import { useState, useCallback, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { invoke } from '@tauri-apps/api/core';

export const useTts = () => {
    const [isSpeaking, setIsSpeaking] = useState(false);
    const [useNeural, setUseNeural] = useState(false);
    const { i18n } = useTranslation();

    // Check if neural TTS is available and handle user preference
    useEffect(() => {
        const checkNeural = async () => {
            const enginePref = localStorage.getItem('murmure_tts_engine') || 'auto';
            
            if (enginePref === 'standard') {
                setUseNeural(false);
                return;
            }

            try {
                const available = await invoke<boolean>('is_neural_tts_available');
                if (enginePref === 'neural') {
                    setUseNeural(available);
                } else {
                    // auto
                    setUseNeural(available);
                }
            } catch {
                setUseNeural(false);
            }
        };

        checkNeural();

        const handleSettingsChange = () => {
            checkNeural();
        };

        window.addEventListener('murmure_tts_settings_changed', handleSettingsChange);
        return () => window.removeEventListener('murmure_tts_settings_changed', handleSettingsChange);
    }, []);

    // Pre-load browser voices for fallback
    useEffect(() => {
        const loadVoices = () => {
            if (window.speechSynthesis) {
                window.speechSynthesis.getVoices();
            }
        };
        loadVoices();
        if (window.speechSynthesis) {
            window.speechSynthesis.onvoiceschanged = loadVoices;
        }
    }, []);

    const stop = useCallback(() => {
        // Stop browser TTS
        if (window.speechSynthesis) {
            window.speechSynthesis.cancel();
        }
        
        // Stop Neural TTS (Kokoro) by notifying backend
        invoke('stop_neural_speak').catch(console.error);
        
        // Instantly update UI
        setIsSpeaking(false);
    }, []);

    const speak = useCallback(
        async (text: string) => {
            if (!text) return;

            const enginePref = localStorage.getItem('murmure_tts_engine') || 'auto';
            const canUseNeural = await invoke<boolean>('is_neural_tts_available');
            
            const shouldTryNeural = enginePref === 'neural' || (enginePref === 'auto' && canUseNeural);

            if (shouldTryNeural) {
                try {
                    console.log(`useTts: Attempting NEURAL TTS (pref: ${enginePref}, available: ${canUseNeural})`);
                    setIsSpeaking(true);
                    await invoke('neural_speak', { text });
                    console.log('useTts: Neural TTS synthesis finished');
                    setIsSpeaking(false);
                    return;
                } catch (e) {
                    console.error('Neural TTS failed:', e);
                    if (enginePref === 'neural') {
                        console.warn('Forced Neural TTS failed, falling back to browser.');
                    }
                }
            } else {
                console.log(`useTts: Skipping neural, using standard browser TTS (pref: ${enginePref}, neural available: ${canUseNeural})`);
            }

            // Fallback to Browser TTS
            if (!window.speechSynthesis) return;

            if (window.speechSynthesis.speaking) {
                stop();
                return;
            }

            const utterance = new SpeechSynthesisUtterance(text);
            utterance.volume = 1;
            utterance.rate = 1;
            utterance.pitch = 1;

            const lang = i18n.language === 'default' ? navigator.language : i18n.language;
            utterance.lang = lang;

            const voices = window.speechSynthesis.getVoices();
            const voice = voices.find((v) => v.lang.startsWith(lang)) 
                       || voices.find((v) => v.lang.startsWith(lang.split('-')[0])) 
                       || voices[0];
            
            if (voice) {
                utterance.voice = voice;
            }

            utterance.onstart = () => setIsSpeaking(true);
            utterance.onend = () => setIsSpeaking(false);
            utterance.onerror = () => setIsSpeaking(false);

            setTimeout(() => {
                window.speechSynthesis.speak(utterance);
                if (window.speechSynthesis.paused) {
                    window.speechSynthesis.resume();
                }
            }, 50);
        },
        [i18n.language, stop]
    );

    const exportWav = useCallback(
        async (text: string, filePath: string) => {
            if (!text || !filePath) return;
            try {
                await invoke('neural_export_wav', { text, filePath });
            } catch (e) {
                console.error('Export WAV failed:', e);
                throw e;
            }
        },
        []
    );

    return { speak, stop, isSpeaking, useNeural, exportWav };
};
