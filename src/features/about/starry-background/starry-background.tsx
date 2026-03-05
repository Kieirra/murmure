import { memo } from 'react';

interface StarryBackgroundProps {
    className?: string;
}

const STAR_COUNT = 18;
const LAYER_COUNT = 2;

// Simple seeded PRNG for deterministic decorative star positions (NOSONAR)
const seededRandom = (seed: number) => {
    const x = Math.sin(seed) * 10000;
    return x - Math.floor(x);
};

let seed = 42;
const nextRandom = () => seededRandom(seed++);

const STARS = Array.from({ length: LAYER_COUNT }, () =>
    Array.from({ length: STAR_COUNT }, () => ({
        cx: `${Math.round(nextRandom() * 10000) / 100}%`,
        cy: `${Math.round(nextRandom() * 10000) / 100}%`,
        r: `${Math.round((nextRandom() + 0.5) * 10) / 10}`,
        delay: `${4 - Math.round(nextRandom() * 50)}s`,
    }))
);

const COMETS = [
    { rotate: -135, cx: '0', cy: '0', delay: '1000ms' },
    { rotate: 20, cx: '100%', cy: '0', delay: '-3.3s' },
    { rotate: 300, cx: '40%', cy: '100%', delay: '-5.5s' },
];

const SVG_FILL_STYLE: React.CSSProperties = {
    position: 'absolute',
    top: 0,
    bottom: 0,
    left: 0,
    right: 0,
};

const StarryBackground = ({ className = '' }: StarryBackgroundProps) => {
    return (
        <div
            className={className}
            style={{
                overflow: 'hidden',
                background:
                    'linear-gradient(to bottom, #18181b, #1f1f2e, #18181b)',
            }}
        >
            <style>{`
                @keyframes starry-twinkle {
                    25% { opacity: 1; }
                }
                @keyframes starry-comet {
                    0% { transform: translateX(0); opacity: 0; }
                    50% { opacity: 1; }
                    100% { transform: translateX(-300vmax); opacity: 0; }
                }
                @media (prefers-reduced-motion: reduce) {
                    .starry-twinkle-anim { animation: none !important; opacity: 0.6 !important; }
                    .starry-comet-anim { animation: none !important; opacity: 0 !important; }
                }
            `}</style>

            {STARS.map((layer, layerIndex) => (
                <svg
                    style={SVG_FILL_STYLE}
                    key={`layer-${layerIndex}`}
                    width="100%"
                    height="100%"
                    preserveAspectRatio="xMidYMid slice"
                    viewBox="0 0 600 600"
                >
                    {layer.map((star) => (
                        <circle
                            className="starry-twinkle-anim"
                            style={{
                                opacity: 0,
                                fill: 'white',
                                animation:
                                    'starry-twinkle 4s ease-in-out infinite',
                                animationDelay: star.delay,
                            }}
                            key={`${star.cx}-${star.cy}`}
                            cx={star.cx}
                            cy={star.cy}
                            r={star.r}
                        />
                    ))}
                </svg>
            ))}

            <svg
                style={SVG_FILL_STYLE}
                width="100%"
                height="100%"
                preserveAspectRatio="xMidYMid slice"
                viewBox="0 0 2500 2500"
            >
                <defs>
                    <radialGradient
                        id="comet-gradient"
                        cx="0"
                        cy="0.5"
                        r="0.5"
                    >
                        <stop
                            offset="0%"
                            stopColor="rgba(255,255,255,.8)"
                        />
                        <stop
                            offset="100%"
                            stopColor="rgba(255,255,255,0)"
                        />
                    </radialGradient>
                </defs>
                {COMETS.map((comet) => (
                    <g key={comet.rotate} transform={`rotate(${comet.rotate})`}>
                        <ellipse
                            className="starry-comet-anim"
                            style={{
                                opacity: 0,
                                animation: 'starry-comet 10s linear infinite',
                                animationDelay: comet.delay,
                            }}
                            fill="url(#comet-gradient)"
                            cx={comet.cx}
                            cy={comet.cy}
                            rx="150"
                            ry="2"
                        />
                    </g>
                ))}
            </svg>
        </div>
    );
};

export default memo(StarryBackground);
