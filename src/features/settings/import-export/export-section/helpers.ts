import { CategoryDefinition, CategorySelection } from '../types';

export const getSelectedCategoryKeys = (definitions: CategoryDefinition[], selection: CategorySelection) => {
    return definitions.filter((def) => selection[def.key]?.selected).map((def) => def.key);
};
