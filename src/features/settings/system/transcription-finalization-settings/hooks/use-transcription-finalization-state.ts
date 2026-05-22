import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { useTranslation } from '@/i18n';
import { AppSettings } from '@/features/settings/settings.types';

const SUPPORTED_TRANSCRIPTION_FINALIZATION_STRATEGIES = ['wav', 'streaming', 'streaming_corrected'] as const;

export type TranscriptionFinalizationStrategy = (typeof SUPPORTED_TRANSCRIPTION_FINALIZATION_STRATEGIES)[number];

export const isTranscriptionFinalizationStrategy = (value: string): value is TranscriptionFinalizationStrategy => {
    return SUPPORTED_TRANSCRIPTION_FINALIZATION_STRATEGIES.some((strategy) => strategy === value);
};

export const useTranscriptionFinalizationState = () => {
    const [transcriptionFinalizationStrategy, setTranscriptionFinalizationStrategyState] =
        useState<TranscriptionFinalizationStrategy>('streaming');
    const { t } = useTranslation();

    useEffect(() => {
        invoke<AppSettings>('get_all_settings').then((settings) => {
            const strategy = settings.transcription_finalization_strategy;
            if (isTranscriptionFinalizationStrategy(strategy)) {
                setTranscriptionFinalizationStrategyState(strategy);
            }
        });
    }, []);

    return {
        transcriptionFinalizationStrategy,
        setTranscriptionFinalizationStrategy: (strategy: TranscriptionFinalizationStrategy) => {
            setTranscriptionFinalizationStrategyState(strategy);
            invoke('set_transcription_finalization_strategy', { strategy }).catch(() => {
                toast.error(t('Failed to save transcription processing mode'));
            });
        },
    };
};
