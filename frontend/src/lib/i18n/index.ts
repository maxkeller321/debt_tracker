import { derived, writable } from 'svelte/store';
import de from './locales/de';
import en from './locales/en';
import es from './locales/es';
import fr from './locales/fr';

export type Locale = 'en' | 'de' | 'es' | 'fr';

const STORAGE_KEY = 'debt-tracker-locale';

const catalogs: Record<Locale, Record<string, string>> = { en, de, es, fr };

export const locales: { code: Locale; flag: string; labelKey: string }[] = [
  { code: 'en', flag: '🇬🇧', labelKey: 'lang.en' },
  { code: 'de', flag: '🇩🇪', labelKey: 'lang.de' },
  { code: 'es', flag: '🇪🇸', labelKey: 'lang.es' },
  { code: 'fr', flag: '🇫🇷', labelKey: 'lang.fr' },
];

function detectLocale(): Locale {
  if (typeof localStorage !== 'undefined') {
    const saved = localStorage.getItem(STORAGE_KEY) as Locale | null;
    if (saved && saved in catalogs) return saved;
  }
  const lang = typeof navigator !== 'undefined' ? navigator.language.slice(0, 2) : 'en';
  if (lang === 'de' || lang === 'es' || lang === 'fr') return lang;
  return 'en';
}

export const locale = writable<Locale>(detectLocale());

locale.subscribe((value) => {
  if (typeof localStorage !== 'undefined') {
    localStorage.setItem(STORAGE_KEY, value);
  }
  if (typeof document !== 'undefined') {
    document.documentElement.lang = value;
  }
});

function interpolate(template: string, vars?: Record<string, string | number>): string {
  if (!vars) return template;
  return Object.entries(vars).reduce(
    (s, [k, v]) => s.replaceAll(`{${k}}`, String(v)),
    template,
  );
}

export const t = derived(locale, ($locale) => {
  const dict = catalogs[$locale];
  const fallback = catalogs.en;
  return (key: string, vars?: Record<string, string | number>) =>
    interpolate(dict[key] ?? fallback[key] ?? key, vars);
});

export function interestMessage(code: string | null, tr: (k: string) => string): string | null {
  if (!code) return null;
  if (code === 'missing_apr') return tr('interest.missingApr');
  return code;
}
