import { motion } from 'framer-motion';

export const ProgressBar = ({ progress }: { progress: number }) => {
    return (
        <div className="sticky top-0 z-50 w-full pb-3">
            <div className="w-full h-1 bg-card rounded-full overflow-hidden">
                <motion.div
                    className="h-full bg-sky-500"
                    initial={{ width: 0 }}
                    animate={{ width: `${progress}%` }}
                    transition={{ duration: 0.5, ease: 'easeInOut' }}
                />
            </div>
        </div>
    );
};
