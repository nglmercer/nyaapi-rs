import {
    Category,
    NyaaOptions,
    SearchByUserOptions,
    SearchOptions,
    SearchResult,
    Torrent,
    TorrentDetail,
} from '../types';
import { parseSearchResults, parseSearchResultsRss, parsePagination } from './search.scraper';
import { parseViewPage } from './view.scraper';
import { parseCategories } from './category.scraper';

export class Nyaa {
    constructor(
        private readonly options: NyaaOptions = {
            baseUrl: 'https://nyaa.si/',
            mode: 'html',
        },
    ) {}

    async search(
        query: string = '',
        options: SearchOptions = {
            page: 1,
            category: 'all',
            filter: 'no filter',
            sort: '',
            order: '',
        },
    ): Promise<SearchResult> {
        const startTime = Date.now();
        const { page, category, filter, sort, order } = options;
        const p = Math.max(1, page || 1);
        const c = this.mapCategory(category);
        const f = this.mapFilter(filter);
        const s = sort === 'date' ? 'id' : sort || '';
        const o = order || '';

        if (this.options.mode === 'rss') {
            const url = `${this.options.baseUrl}?page=rss&q=${query}&c=${c}&f=${f}&p=${p}&s=${s}&o=${o}`;
            const res = await fetch(url).then(r => r.text());
            const torrents = parseSearchResultsRss(res);
            return {
                data: torrents,
                total: 0,
                page: 0,
                totalPage: 0,
                perPage: 0,
                range: null,
                nextPage: false,
                timeTaken: Date.now() - startTime,
            };
        }

        const url = `${this.options.baseUrl}?&q=${query}&c=${c}&f=${f}&p=${p}&s=${s}&o=${o}`;
        const res = await fetch(url).then(r => r.text());
        const torrents = parseSearchResults(res);
        const pagination = query ? parsePagination(res) : null;

        return {
            data: torrents,
            total: pagination?.total ?? null,
            page: p,
            totalPage: pagination?.totalPage ?? null,
            perPage: torrents.length,
            range: pagination?.range ?? null,
            nextPage: pagination?.nextPage ?? false,
            timeTaken: Date.now() - startTime,
        };
    }

    async searchByUser(
        username: string,
        options: SearchByUserOptions = {
            page: 1,
            category: 'all',
            filter: 'no filter',
            sort: '',
            order: '',
            query: '',
        },
    ): Promise<Torrent[]> {
        const startTime = Date.now();
        const { page, category, filter, sort, order, query } = options;
        const p = Math.max(1, page || 1);
        const c = this.mapCategory(category);
        const f = this.mapFilter(filter);
        const s = sort === 'date' ? 'id' : sort || '';
        const o = order || '';
        const q = query || '';

        if (this.options.mode === 'rss') {
            const url = `${this.options.baseUrl}?page=rss&u=${username}&q=${q}&c=${c}&f=${f}&p=${p}&s=${s}&o=${o}`;
            const res = await fetch(url).then(r => r.text());
            return parseSearchResultsRss(res);
        }

        const url = `${this.options.baseUrl}?&u=${username}&q=${q}&c=${c}&f=${f}&p=${p}&s=${s}&o=${o}`;
        const res = await fetch(url).then(r => r.text());
        return parseSearchResults(res);
    }

    async view(id: number): Promise<TorrentDetail | null> {
        const baseUrl = this.options.baseUrl.replace(/\/$/, '');
        const url = `${baseUrl}/view/${id}`;
        const res = await fetch(url).then(r => r.text());
        return parseViewPage(res, id);
    }

    async viewFromTorrent(torrent: Torrent): Promise<TorrentDetail | null> {
        if (!torrent.viewUrl) {
            if (torrent.id) {
                return this.view(torrent.id);
            }
            return null;
        }
        const baseUrl = this.options.baseUrl.replace(/\/$/, '');
        const fullUrl = torrent.viewUrl.startsWith('/') 
            ? `${baseUrl}${torrent.viewUrl}` 
            : torrent.viewUrl;
        const res = await fetch(fullUrl).then(r => r.text());
        return parseViewPage(res, torrent.id);
    }

    async getCategories(): Promise<Category[]> {
        const baseUrl = this.options.baseUrl.replace(/\/$/, '');
        const url = `${baseUrl}/`;
        const res = await fetch(url).then(r => r.text());
        return parseCategories(res);
    }

    private mapCategory(category: string | undefined): string {
        const map: Record<string, string> = {
            anime: '1_0',
            audio: '2_0',
            literature: '3_0',
            'live-action': '4_0',
            pictures: '5_0',
            software: '6_0',
            games: '7_0',
        };
        return map[category || 'all'] || '0_0';
    }

    private mapFilter(filter: string | undefined): string {
        const map: Record<string, string> = {
            'trusted only': '2',
            'no remakes': '1',
        };
        return map[filter || 'no filter'] || '0';
    }
}

export default Nyaa;