import React from 'react';
import { createRoot } from 'react-dom/client';
import { SmartMic } from './smartmic';
import '../tailwind.css';

const root = document.getElementById('root')!;
createRoot(root).render(
    <React.StrictMode>
        <SmartMic />
    </React.StrictMode>
);
