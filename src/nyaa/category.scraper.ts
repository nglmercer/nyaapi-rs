import * as cheerio from 'cheerio';
import { Category } from '../types';

export const parseCategories = (html: string): Category[] => {
    const $ = cheerio.load(html);
    const categories: Category[] = [];
    const seenIds = new Set<string>();

    $('select[name="c"] option').each((_, el) => {
        const value = $(el).attr('value');
        const text = $(el).text().trim();

        if (value && text && value !== '0_0' && !seenIds.has(value)) {
            seenIds.add(value);
            const isSubCategory = text.startsWith('- ');
            const name = isSubCategory ? text.slice(2) : text;

            if (!isSubCategory) {
                categories.push({
                    id: value,
                    name,
                    subCategories: [],
                });
            } else {
                const parentIndex = categories.length - 1;
                if (parentIndex >= 0) {
                    categories[parentIndex].subCategories?.push({
                        id: value,
                        name,
                    });
                }
            }
        }
    });

    return categories;
};