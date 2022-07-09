import { parse_wasm } from 'text-to-sounds';

const MAX_TEXT_LENGTH = 5000;

const contenteditableEl = document.getElementById('contenteditable');
const maxLengthTextEl = document.getElementById('max-length-text');

const contenteditableFocus = () => {
    contenteditableEl.focus();
};

const contenteditableFontSize = () => {
    const textContentLength = contenteditableEl.textContent.length;

    if (textContentLength > 150) {
        contenteditableEl.style.fontSize = 24 + 'px';
        return;
    }

    if (textContentLength > 50) {
        contenteditableEl.style.fontSize = 50 + 'px';
        return;
    }

    contenteditableEl.style.fontSize = 100 + 'px';
};

const contenteditableHighlight = () => {
    const textContent = contenteditableEl.textContent;

    const isMaxTextLength = textContent.length >= MAX_TEXT_LENGTH;

    const sounds = parse_wasm(textContent.slice(0, MAX_TEXT_LENGTH));

    let soundsHtml = sounds.reduce((acc, sound) => {
        if (sound.kind === 'Undefined') {
            acc += sound.text;
            return acc;
        }

        acc += `<span class="${sound.kind}">${sound.text}</span>`;
        return acc;
    }, '');

    contenteditableEl.innerHTML = soundsHtml;
    document.execCommand('selectAll', false, null);
    document.getSelection().collapseToEnd();

    if (isMaxTextLength) {
        maxLengthTextEl.classList.remove('hidden');
    } else {
        maxLengthTextEl.classList.add('hidden');
    }
};

contenteditableFocus();
contenteditableFontSize();

contenteditableEl.addEventListener('input', () => {
    contenteditableFontSize();
    contenteditableHighlight();
});

