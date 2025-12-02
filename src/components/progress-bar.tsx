import { motion } from 'framer-motion';

export const ProgressBar = ({ progress }: { progress: number }) => {
    return (
        <div className="w-full h-1 bg-zinc-800 rounded-full mb-8 overflow-hidden">
            <motion.div
                className="h-full bg-sky-500"
                initial={{ width: 0 }}
                animate={{ width: `${progress}%` }}
                transition={{ duration: 0.5, ease: 'easeInOut' }}
            />
        </div>
    );
};
