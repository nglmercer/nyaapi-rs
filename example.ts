import { Nyaa } from './src';

async function main() {
    const nyaa = new Nyaa({
        baseUrl: 'https://nyaa.si',
        mode: 'html',
    });

    console.log('Search for torrents:');
    const result = await nyaa.search('One Piece', {
        page: 1,
        category: 'anime',
        filter: 'no filter',
        sort: 'date',
        order: 'desc',
    });

    console.log('result', result);

    console.log('Search by user:');
    const userResult = await nyaa.searchByUser('Fan-Kai', {
        query: 'One Piece',
        category: 'anime',
    });

    console.log(`userResult`, userResult);

    console.log('\nGet torrent detail:');
    const detail = await nyaa.view(2099890);
    console.log('detail', detail);

    console.log('\nGet torrent detail from search result (viewUrl):');
    const firstTorrent = result.data[0];
    const detailFromTorrent = await nyaa.viewFromTorrent(firstTorrent);
    console.log('detailFromTorrent', detailFromTorrent);

    console.log('\nGet categories:');
    const categories = await nyaa.getCategories();
    console.log('categories', JSON.stringify(categories, null, 2));

    console.log('\nUsing RSS mode:');
    const rssNyaa = new Nyaa({
        baseUrl: 'https://nyaa.land',
        mode: 'rss',
    });

    const rssResult = await rssNyaa.search('Anime');
    console.log(`rssResult`, rssResult);
}

main();
