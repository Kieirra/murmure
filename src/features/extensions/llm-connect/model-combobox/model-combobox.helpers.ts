import { OllamaModel } from '../hooks/use-llm-connect';

export const filterModels = (models: OllamaModel[], query: string) => {
    const normalized = query.trim().toLowerCase();
    if (normalized.length === 0) {
        return models;
    }
    return models.filter((model) => model.name.toLowerCase().includes(normalized));
};

export const shouldOfferCustomModel = (models: OllamaModel[], query: string) => {
    const trimmed = query.trim();
    if (trimmed.length === 0) {
        return false;
    }
    return !models.some((model) => model.name === trimmed);
};
