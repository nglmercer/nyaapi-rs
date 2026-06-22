import * as cheerio from 'cheerio';
import { TorrentDetail, TorrentFile } from '../types';

export const parseViewPage = (html: string, id: number): TorrentDetail | null => {
    const $ = cheerio.load(html);

    const title = $('.panel h3').first().text().trim();
    if (!title) return null;

    const titleParts = title.split('\n');
    const cleanTitle = titleParts[0].trim();

    const getRowText = (label: string): string | null => {
        const row = $(`.panel-body .row`).filter((_, el) => $(el).text().includes(label));
        if (!row.length) return null;
        const colMd5 = row.find('.col-md-5');
        return colMd5.eq(0).text().trim() || null;
    };

    const getRowLink = (label: string): string | null => {
        const row = $(`.panel-body .row`).filter((_, el) => $(el).text().includes(label));
        if (!row.length) return null;
        const link = row.find('.col-md-5 a');
        return link.attr('href') || null;
    };

    const parseCategory = (): { category: string; subCategory: string } => {
        const categoryCol = getRowText('Category:');
        if (!categoryCol) return { category: '', subCategory: '' };
        const parts = categoryCol.split(' - ');
        return {
            category: parts[0] || '',
            subCategory: parts[1] || '',
        };
    };

    const categoryInfo = parseCategory();

    const tsAttr = $('[data-timestamp]').attr('data-timestamp');
    const date = tsAttr ? new Date(parseInt(tsAttr) * 1000) : new Date();

    const submitter = getRowText('Submitter:');
    const submitterLink = $('a.text-default');
    const submitterId = submitterLink.attr('href')?.replace('/user/', '') || undefined;
    const information = getRowLink('Information:') || undefined;
    const fileSize = getRowText('File size:') || '';
    const completed = parseInt(getRowText('Completed:') || '0') || undefined;
    const seeders = parseInt($('[style*="color: green"]').text().trim()) || 0;
    const leechers = parseInt($('[style*="color: red"]').text().trim()) || 0;

    const infoHashEl = $('kbd');
    const hash = infoHashEl.text().trim() || undefined;

    const magnet = $('a[href^="magnet:"]').attr('href') || '';
    const torrentUrl = $('a[href$=".torrent"]').attr('href') || '';
    const magnetUrl = $('a[href^="magnet?"]').attr('href') || '';
    const downloadLink = magnet || (magnetUrl ? `magnet:?${magnetUrl.split('?')[1]}` : '') || torrentUrl || '';

    const description = $('#torrent-description').html() || '';

    const files: TorrentFile[] = [];
    $('.torrent-file-list li').each((_, el) => {
        const fileName = $(el).clone().children().remove().end().text().trim();
        const fileSizeMatch = $(el).find('.file-size').text().trim();
        if (fileName) {
            files.push({ name: fileName, size: fileSizeMatch });
        }
    });

    const commentsText = $('.panel-heading a[data-toggle="collapse"]').text() || '';
    const commentsMatch = commentsText.match(/Comments\s*-\s*(\d+)/);
    const comments = commentsMatch ? parseInt(commentsMatch[1]) : 0;

    return {
        id,
        title: cleanTitle,
        name: cleanTitle,
        category: categoryInfo.category,
        subCategory: categoryInfo.subCategory,
        date,
        seeders,
        leechers,
        downloads: completed || 0,
        completed,
        magnet: downloadLink || magnet,
        size: fileSize,
        hash,
        submitter: submitter || undefined,
        submitterId,
        information,
        description,
        files,
        comments,
    };
};