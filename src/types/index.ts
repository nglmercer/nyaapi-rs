export interface Torrent {
    id: number;
    name: string;
    magnet: string;
    size: string;
    category: string;
    subCategory?: string;
    date: Date;
    seeders: number;
    leechers: number;
    downloads: number;
    hash?: string;
    submitter?: string;
    submitterId?: string;
    information?: string;
    completed?: number;
    description?: string;
    torrentUrl?: string;
    viewUrl?: string;
    comments?: number;
}

export interface TorrentFile {
    name: string;
    size: string;
}

export interface TorrentDetail extends Omit<Torrent, 'category'> {
    title: string;
    category: string;
    subCategory: string;
    files: TorrentFile[];
    description: string;
    information?: string;
    comments: number;
}

export interface SearchResult {
    data: Torrent[];
    total: number | null;
    page: number;
    totalPage: number | null;
    perPage: number;
    nextPage: boolean;
    range: string | null;
    timeTaken: number;
}

export interface PaginationInfo {
    total: number | null;
    totalPage: number | null;
    page: number;
    perPage: number;
    range: string | null;
    nextPage: boolean;
}

export interface SearchOptions {
    page?: number;
    category?:
        | 'all'
        | 'anime'
        | 'audio'
        | 'literature'
        | 'live-action'
        | 'pictures'
        | 'software'
        | 'games';
    filter?: 'no filter' | 'trusted only' | 'no remakes';
    sort?: 'comments' | 'size' | 'date' | 'seeders' | 'leechers' | 'downloads' | '';
    order?: 'asc' | 'desc' | '';
}

export interface SearchByUserOptions extends SearchOptions {
    query?: string;
}

export interface NyaaOptions {
    baseUrl: string;
    mode: 'rss' | 'html';
}

export interface Category {
    id: string;
    name: string;
    subCategories?: Category[];
}