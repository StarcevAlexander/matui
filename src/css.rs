pub fn stylesheet() -> String {
    String::from(
        r#"
:root {
  --matui-primary: #6750a4;
  --matui-on-primary: #ffffff;
  --matui-primary-container: #eaddff;
  --matui-on-primary-container: #21005d;
  --matui-secondary: #625b71;
  --matui-on-secondary: #ffffff;
  --matui-secondary-container: #e8def8;
  --matui-on-secondary-container: #1d192b;
  --matui-tertiary: #7d5260;
  --matui-on-tertiary: #ffffff;
  --matui-tertiary-container: #ffd8e4;
  --matui-on-tertiary-container: #31111d;
  --matui-error: #b3261e;
  --matui-on-error: #ffffff;
  --matui-error-container: #f9dedc;
  --matui-on-error-container: #410e0b;
  --matui-surface: #fef7ff;
  --matui-surface-dim: #ded8e1;
  --matui-surface-container-lowest: #ffffff;
  --matui-surface-container-low: #f7f2fa;
  --matui-surface-container: #f3edf7;
  --matui-surface-container-high: #ece6f0;
  --matui-surface-container-highest: #e6e0e9;
  --matui-on-surface: #1d1b20;
  --matui-on-surface-variant: #49454f;
  --matui-outline: #79747e;
  --matui-outline-variant: #cac4d0;
  --matui-inverse-surface: #322f35;
  --matui-inverse-on-surface: #f5eff7;
  --matui-inverse-primary: #d0bcff;
  --matui-scrim: rgba(0, 0, 0, 0.4);
  --matui-font: "Roboto", "Segoe UI", system-ui, sans-serif;
}

.matui-display-large { font-family: var(--matui-font); font-size: 3.5625rem; line-height: 4rem; font-weight: 400; letter-spacing: -0.25px; }
.matui-display-medium { font-family: var(--matui-font); font-size: 2.8125rem; line-height: 3.25rem; font-weight: 400; }
.matui-display-small { font-family: var(--matui-font); font-size: 2.25rem; line-height: 2.75rem; font-weight: 400; }
.matui-headline-large { font-family: var(--matui-font); font-size: 2rem; line-height: 2.5rem; font-weight: 400; }
.matui-headline-medium { font-family: var(--matui-font); font-size: 1.75rem; line-height: 2.25rem; font-weight: 400; }
.matui-headline-small { font-family: var(--matui-font); font-size: 1.5rem; line-height: 2rem; font-weight: 400; }
.matui-title-large { font-family: var(--matui-font); font-size: 1.375rem; line-height: 1.75rem; font-weight: 400; }
.matui-title-medium { font-family: var(--matui-font); font-size: 1rem; line-height: 1.5rem; font-weight: 500; letter-spacing: 0.15px; }
.matui-title-small { font-family: var(--matui-font); font-size: 0.875rem; line-height: 1.25rem; font-weight: 500; letter-spacing: 0.1px; }
.matui-body-large { font-family: var(--matui-font); font-size: 1rem; line-height: 1.5rem; font-weight: 400; letter-spacing: 0.5px; }
.matui-body-medium { font-family: var(--matui-font); font-size: 0.875rem; line-height: 1.25rem; font-weight: 400; letter-spacing: 0.25px; }
.matui-body-small { font-family: var(--matui-font); font-size: 0.75rem; line-height: 1rem; font-weight: 400; letter-spacing: 0.4px; }
.matui-label-large { font-family: var(--matui-font); font-size: 0.875rem; line-height: 1.25rem; font-weight: 500; letter-spacing: 0.1px; }
.matui-label-medium { font-family: var(--matui-font); font-size: 0.75rem; line-height: 1rem; font-weight: 500; letter-spacing: 0.5px; }
.matui-label-small { font-family: var(--matui-font); font-size: 0.6875rem; line-height: 1rem; font-weight: 500; letter-spacing: 0.5px; }

.matui-elevation-0 { box-shadow: none; }
.matui-elevation-1 { box-shadow: 0 1px 2px rgba(0, 0, 0, 0.14), 0 1px 3px 1px rgba(0, 0, 0, 0.08); }
.matui-elevation-2 { box-shadow: 0 1px 2px rgba(0, 0, 0, 0.14), 0 2px 6px 2px rgba(0, 0, 0, 0.08); }
.matui-elevation-3 { box-shadow: 0 4px 8px 3px rgba(0, 0, 0, 0.15), 0 1px 3px rgba(0, 0, 0, 0.3); }
.matui-elevation-4 { box-shadow: 0 6px 10px 4px rgba(0, 0, 0, 0.15), 0 2px 3px rgba(0, 0, 0, 0.3); }
.matui-elevation-5 { box-shadow: 0 8px 12px 6px rgba(0, 0, 0, 0.15), 0 4px 4px rgba(0, 0, 0, 0.3); }

.matui-overline { font-family: var(--matui-font); font-size: 0.75rem; font-weight: 600; letter-spacing: 0.5px; text-transform: uppercase; color: var(--matui-on-surface-variant); margin: 0 0 8px; }

.matui-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 10px 24px;
  border: none;
  border-radius: 100px;
  font-family: var(--matui-font);
  font-size: 0.875rem;
  font-weight: 500;
  letter-spacing: 0.1px;
  line-height: 1.25rem;
  cursor: pointer;
  text-decoration: none;
  position: relative;
  background: var(--matui-btn-bg, var(--matui-primary));
  color: var(--matui-btn-fg, var(--matui-on-primary));
  transition: box-shadow 0.2s ease, filter 0.15s ease;
}

.matui-btn--primary { --matui-btn-bg: var(--matui-primary); --matui-btn-fg: var(--matui-on-primary); --matui-btn-outline: var(--matui-primary); --matui-btn-tonal: var(--matui-primary-container); --matui-btn-tonal-fg: var(--matui-on-primary-container); }
.matui-btn--secondary { --matui-btn-bg: var(--matui-secondary); --matui-btn-fg: var(--matui-on-secondary); --matui-btn-outline: var(--matui-secondary); --matui-btn-tonal: var(--matui-secondary-container); --matui-btn-tonal-fg: var(--matui-on-secondary-container); }
.matui-btn--tertiary { --matui-btn-bg: var(--matui-tertiary); --matui-btn-fg: var(--matui-on-tertiary); --matui-btn-outline: var(--matui-tertiary); --matui-btn-tonal: var(--matui-tertiary-container); --matui-btn-tonal-fg: var(--matui-on-tertiary-container); }
.matui-btn--error { --matui-btn-bg: var(--matui-error); --matui-btn-fg: var(--matui-on-error); --matui-btn-outline: var(--matui-error); --matui-btn-tonal: var(--matui-error-container); --matui-btn-tonal-fg: var(--matui-on-error-container); }

.matui-btn--filled:hover:not(:disabled) { box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2); filter: brightness(1.06); }
.matui-btn--filled:active:not(:disabled) { filter: brightness(0.94); box-shadow: none; }
.matui-btn--tonal { background: var(--matui-btn-tonal); color: var(--matui-btn-tonal-fg); }
.matui-btn--tonal:hover:not(:disabled) { filter: brightness(0.97); box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15); }
.matui-btn--tonal:active:not(:disabled) { filter: brightness(0.92); box-shadow: none; }
.matui-btn--outlined { background: transparent; color: var(--matui-btn-bg); border: 1px solid var(--matui-btn-outline); }
.matui-btn--outlined:hover:not(:disabled) { background: color-mix(in srgb, var(--matui-btn-bg) 8%, transparent); }
.matui-btn--text { background: transparent; color: var(--matui-btn-bg); padding: 10px 12px; }
.matui-btn--text:hover:not(:disabled) { background: color-mix(in srgb, var(--matui-btn-bg) 8%, transparent); }
.matui-btn:focus-visible { outline: 3px solid color-mix(in srgb, var(--matui-btn-bg) 40%, transparent); outline-offset: 2px; }
.matui-btn--disabled, .matui-btn:disabled { opacity: 0.38; cursor: not-allowed; pointer-events: none; }

.matui-segmented { display: inline-flex; font-family: var(--matui-font); }
.matui-segmented__btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 10px 18px;
  border: 1px solid var(--matui-outline);
  background: transparent;
  color: var(--matui-on-surface);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
}
.matui-segmented__btn:first-child { border-radius: 100px 0 0 100px; border-right: none; }
.matui-segmented__btn:last-child { border-radius: 0 100px 100px 0; border-left: none; }
.matui-segmented__btn--selected { background: var(--matui-secondary-container); color: var(--matui-on-secondary-container); border-color: transparent; }
.matui-segmented__btn + .matui-segmented__btn--selected { border-left: 1px solid transparent; }
.matui-segmented__btn:disabled { opacity: 0.38; cursor: not-allowed; }

.matui-icon-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 100px;
  cursor: pointer;
  font-size: 1.25rem;
  line-height: 1;
  background: transparent;
  color: var(--matui-on-surface-variant);
  transition: background 0.15s ease, filter 0.15s ease;
}

.matui-icon-btn:hover:not(:disabled) { background: color-mix(in srgb, var(--matui-on-surface-variant) 10%, transparent); }
.matui-icon-btn:focus-visible { outline: 2px solid var(--matui-primary); outline-offset: 2px; }
.matui-icon-btn--filled { background: var(--matui-btn-bg, var(--matui-primary)); color: var(--matui-btn-fg, var(--matui-on-primary)); }
.matui-icon-btn--tonal { background: var(--matui-btn-tonal, var(--matui-secondary-container)); color: var(--matui-btn-tonal-fg, var(--matui-on-secondary-container)); }
.matui-icon-btn--outlined { background: transparent; border: 1px solid var(--matui-outline); color: var(--matui-on-surface-variant); }
.matui-icon-btn--primary { --matui-btn-bg: var(--matui-primary); --matui-btn-fg: var(--matui-on-primary); --matui-btn-tonal: var(--matui-primary-container); --matui-btn-tonal-fg: var(--matui-on-primary-container); }
.matui-icon-btn--secondary { --matui-btn-bg: var(--matui-secondary); --matui-btn-fg: var(--matui-on-secondary); --matui-btn-tonal: var(--matui-secondary-container); --matui-btn-tonal-fg: var(--matui-on-secondary-container); }
.matui-icon-btn--tertiary { --matui-btn-bg: var(--matui-tertiary); --matui-btn-fg: var(--matui-on-tertiary); --matui-btn-tonal: var(--matui-tertiary-container); --matui-btn-tonal-fg: var(--matui-on-tertiary-container); }
.matui-icon-btn--error { --matui-btn-bg: var(--matui-error); --matui-btn-fg: var(--matui-on-error); --matui-btn-tonal: var(--matui-error-container); --matui-btn-tonal-fg: var(--matui-on-error-container); }
.matui-icon-btn:disabled { opacity: 0.38; cursor: not-allowed; }

.matui-card {
  background: var(--matui-surface);
  color: var(--matui-on-surface);
  border-radius: 12px;
  padding: 16px 16px 20px;
  border: 1px solid var(--matui-outline-variant);
  font-family: var(--matui-font);
  transition: box-shadow 0.2s ease;
}

.matui-card--elevated { border: none; background: var(--matui-surface-container-low); box-shadow: 0 1px 2px rgba(0, 0, 0, 0.14), 0 1px 3px 1px rgba(0, 0, 0, 0.08); }
.matui-card--elevated:hover { box-shadow: 0 2px 6px 2px rgba(0, 0, 0, 0.09), 0 1px 2px rgba(0, 0, 0, 0.14); }

.matui-card__title { margin: 0 0 8px; font-size: 1rem; line-height: 1.5rem; font-weight: 500; }
.matui-card__body { font-size: 0.875rem; line-height: 1.25rem; color: var(--matui-on-surface-variant); }

.matui-fab {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  width: 56px;
  height: 56px;
  border: none;
  border-radius: 16px;
  cursor: pointer;
  font-family: var(--matui-font);
  background: var(--matui-btn-bg, var(--matui-primary-container));
  color: var(--matui-btn-fg, var(--matui-on-primary-container));
  box-shadow: 0 4px 8px 3px rgba(0, 0, 0, 0.15), 0 1px 3px rgba(0, 0, 0, 0.3);
  transition: box-shadow 0.2s ease, filter 0.15s ease;
}

.matui-fab:hover:not(:disabled) { box-shadow: 0 6px 10px 4px rgba(0, 0, 0, 0.15), 0 2px 3px rgba(0, 0, 0, 0.3); filter: brightness(1.03); }
.matui-fab:active:not(:disabled) { filter: brightness(0.95); box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3); }

.matui-fab--primary { --matui-btn-bg: var(--matui-primary-container); --matui-btn-fg: var(--matui-on-primary-container); }
.matui-fab--secondary { --matui-btn-bg: var(--matui-secondary-container); --matui-btn-fg: var(--matui-on-secondary-container); }
.matui-fab--tertiary { --matui-btn-bg: var(--matui-tertiary-container); --matui-btn-fg: var(--matui-on-tertiary-container); }
.matui-fab--error { --matui-btn-bg: var(--matui-error); --matui-btn-fg: var(--matui-on-error); }

.matui-fab__icon { font-size: 1.375rem; line-height: 1; }
.matui-fab__label { font-size: 0.875rem; font-weight: 600; letter-spacing: 0.1px; }
.matui-fab:has(.matui-fab__label) { width: auto; padding: 0 20px; }
.matui-fab:disabled { opacity: 0.38; cursor: not-allowed; }

.matui-topbar {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px 24px;
  background: var(--matui-primary);
  color: var(--matui-on-primary);
  font-family: var(--matui-font);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.matui-topbar--center { justify-content: center; }
.matui-topbar--large { padding: 24px; flex-direction: column; align-items: flex-start; gap: 8px; }
.matui-topbar__title { margin: 0; font-size: 1.375rem; font-weight: 500; letter-spacing: 0.15px; }
.matui-topbar--large .matui-topbar__title { font-size: 2rem; }
.matui-topbar__actions { margin-left: auto; display: flex; gap: 8px; }
.matui-topbar--large .matui-topbar__actions { margin-left: 0; }

.matui-bottom-appbar {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 16px;
  background: var(--matui-surface-container);
  color: var(--matui-on-surface);
  font-family: var(--matui-font);
  box-shadow: 0 2px 6px 2px rgba(0, 0, 0, 0.1);
  z-index: 5;
}
.matui-bottom-appbar__section { display: flex; align-items: center; gap: 8px; }

.matui-textfield { display: grid; gap: 4px; max-width: 280px; font-family: var(--matui-font); position: relative; }
.matui-textfield__input {
  padding: 16px 16px 6px;
  border: 1px solid var(--matui-outline);
  border-radius: 8px 8px 4px 4px;
  font: inherit;
  font-size: 0.95rem;
  background: var(--matui-surface-container-high);
  color: var(--matui-on-surface);
  outline: none;
}
.matui-textfield__input:focus { border-color: var(--matui-primary); border-bottom-width: 2px; padding-bottom: 5px; }
.matui-textfield__label {
  position: absolute;
  left: 16px;
  top: 16px;
  font-size: 0.95rem;
  color: var(--matui-on-surface-variant);
  pointer-events: none;
  transition: all 0.15s ease;
}
.matui-textfield__input:focus + .matui-textfield__label,
.matui-textfield__input:not(:placeholder-shown) + .matui-textfield__label { top: 6px; font-size: 0.7rem; color: var(--matui-primary); }
.matui-textfield--outlined .matui-textfield__input { background: transparent; border-radius: 8px; padding: 12px 16px; }
.matui-textfield--outlined .matui-textfield__label { top: 12px; }
.matui-textfield--outlined .matui-textfield__input:focus + .matui-textfield__label,
.matui-textfield--outlined .matui-textfield__input:not(:placeholder-shown) + .matui-textfield__label { top: -8px; background: var(--matui-surface); padding: 0 6px; }
.matui-textfield--error .matui-textfield__input { border-color: var(--matui-error); }
.matui-textfield--error .matui-textfield__label { color: var(--matui-error); }
.matui-textfield__helper { font-size: 0.75rem; color: var(--matui-on-surface-variant); padding-left: 16px; }
.matui-textfield--error .matui-textfield__helper { color: var(--matui-error); }
.matui-textfield__input:disabled { opacity: 0.38; }

.matui-checkbox { display: inline-flex; align-items: center; gap: 12px; cursor: pointer; font-family: var(--matui-font); font-size: 0.875rem; color: var(--matui-on-surface); position: relative; }
.matui-checkbox input { position: absolute; opacity: 0; width: 0; height: 0; }
.matui-checkbox__box {
  width: 18px;
  height: 18px;
  border: 2px solid var(--matui-on-surface-variant);
  border-radius: 3px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}
.matui-checkbox__box::after { content: ""; width: 9px; height: 5px; border-left: 2px solid var(--matui-on-primary); border-bottom: 2px solid var(--matui-on-primary); transform: rotate(-45deg) scale(0); transition: transform 0.15s ease; }
.matui-checkbox input:checked + .matui-checkbox__box { background: var(--matui-primary); border-color: var(--matui-primary); }
.matui-checkbox input:checked + .matui-checkbox__box::after { transform: rotate(-45deg) scale(1); }
.matui-checkbox input:indeterminate + .matui-checkbox__box { background: var(--matui-primary); border-color: var(--matui-primary); }
.matui-checkbox input:disabled + .matui-checkbox__box { opacity: 0.38; }

.matui-radio { display: inline-flex; align-items: center; gap: 12px; cursor: pointer; font-family: var(--matui-font); font-size: 0.875rem; color: var(--matui-on-surface); position: relative; }
.matui-radio input { position: absolute; opacity: 0; width: 0; height: 0; }
.matui-radio__circle {
  width: 20px;
  height: 20px;
  border: 2px solid var(--matui-on-surface-variant);
  border-radius: 100px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}
.matui-radio__circle::after { content: ""; width: 10px; height: 10px; border-radius: 100px; background: var(--matui-primary); transform: scale(0); transition: transform 0.15s ease; }
.matui-radio input:checked + .matui-radio__circle { border-color: var(--matui-primary); }
.matui-radio input:checked + .matui-radio__circle::after { transform: scale(1); }
.matui-radio input:disabled + .matui-radio__circle { opacity: 0.38; }

.matui-switch { display: inline-flex; align-items: center; gap: 12px; cursor: pointer; font-family: var(--matui-font); font-size: 0.875rem; color: var(--matui-on-surface); position: relative; }
.matui-switch input { position: absolute; opacity: 0; width: 0; height: 0; }
.matui-switch__track {
  width: 52px;
  height: 32px;
  border-radius: 100px;
  background: var(--matui-surface-container-highest);
  border: 2px solid var(--matui-outline);
  display: inline-flex;
  align-items: center;
  padding: 2px;
  transition: all 0.2s ease;
  box-sizing: border-box;
}
.matui-switch__thumb {
  width: 16px;
  height: 16px;
  border-radius: 100px;
  background: var(--matui-outline);
  transition: all 0.2s ease;
}
.matui-switch input:checked + .matui-switch__track { background: var(--matui-primary); border-color: var(--matui-primary); }
.matui-switch input:checked + .matui-switch__track .matui-switch__thumb { transform: translateX(20px); width: 24px; height: 24px; background: var(--matui-on-primary); }
.matui-switch input:disabled + .matui-switch__track { opacity: 0.38; }

.matui-slider {
  appearance: none;
  width: 240px;
  height: 6px;
  border-radius: 100px;
  background: linear-gradient(to right, var(--matui-primary) var(--matui-slider-value, 50%), var(--matui-surface-container-highest) var(--matui-slider-value, 50%));
  outline: none;
  cursor: pointer;
}
.matui-slider::-webkit-slider-thumb {
  appearance: none;
  width: 20px;
  height: 20px;
  border-radius: 100px;
  background: var(--matui-primary);
  border: 2px solid var(--matui-surface);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}
.matui-slider::-moz-range-thumb { width: 20px; height: 20px; border-radius: 100px; background: var(--matui-primary); border: 2px solid var(--matui-surface); }
.matui-slider:disabled { opacity: 0.38; cursor: not-allowed; }

.matui-chip {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 7px 16px;
  border-radius: 8px;
  border: 1px solid var(--matui-outline-variant);
  background: var(--matui-surface-container-low);
  color: var(--matui-on-surface-variant);
  font-family: var(--matui-font);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.15s ease;
}
.matui-chip:hover { background: var(--matui-surface-container-high); }
.matui-chip--selected { background: var(--matui-secondary-container); color: var(--matui-on-secondary-container); border-color: transparent; }
.matui-chip__icon { font-size: 1rem; line-height: 1; }

.matui-divider { height: 1px; background: var(--matui-outline-variant); border: none; margin: 8px 0; }
.matui-divider--inset { margin-left: 56px; }
.matui-divider--vertical { width: 1px; height: auto; align-self: stretch; margin: 0 8px; }

.matui-progress {
  width: 240px;
  height: 4px;
  border-radius: 100px;
  background: color-mix(in srgb, var(--matui-primary) 15%, transparent);
  overflow: hidden;
  position: relative;
}
.matui-progress__bar { position: absolute; inset: 0; background: var(--matui-primary); transform-origin: left; animation: matui-indeterminate 1.6s ease-in-out infinite; }
.matui-progress--determinate .matui-progress__bar { animation: none; transform: scaleX(var(--matui-progress-value, 0.5)); transition: transform 0.3s ease; }
@keyframes matui-indeterminate { 0% { transform: translateX(-100%) scaleX(0.4); } 50% { transform: translateX(30%) scaleX(0.5); } 100% { transform: translateX(110%) scaleX(0.4); } }

.matui-spinner {
  width: 40px;
  height: 40px;
  border-radius: 100px;
  border: 4px solid color-mix(in srgb, var(--matui-primary) 15%, transparent);
  border-top-color: var(--matui-primary);
  animation: matui-spin 1s linear infinite;
}
@keyframes matui-spin { to { transform: rotate(360deg); } }

.matui-list-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px 16px;
  font-family: var(--matui-font);
  background: var(--matui-surface);
  color: var(--matui-on-surface);
}
.matui-list-item:hover { background: var(--matui-surface-container); }
.matui-list-item__leading { font-size: 1.375rem; color: var(--matui-on-surface-variant); }
.matui-list-item__text { flex: 1; display: grid; gap: 2px; }
.matui-list-item__headline { font-size: 0.95rem; font-weight: 500; }
.matui-list-item__supporting { font-size: 0.8125rem; color: var(--matui-on-surface-variant); }
.matui-list-item__trailing { font-size: 1rem; color: var(--matui-on-surface-variant); }

.matui-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 16px;
  height: 16px;
  padding: 0 4px;
  border-radius: 100px;
  background: var(--matui-error);
  color: var(--matui-on-error);
  font-family: var(--matui-font);
  font-size: 0.6875rem;
  font-weight: 500;
}
.matui-badge--dot { min-width: 6px; width: 6px; height: 6px; padding: 0; }

.matui-dialog-scrim {
  display: none;
  position: fixed;
  inset: 0;
  background: var(--matui-scrim);
  align-items: center;
  justify-content: center;
  z-index: 10;
}
.matui-dialog-scrim--open { display: flex; }
.matui-dialog-scrim--preview { position: static; background: transparent; }
.matui-dialog {
  background: var(--matui-surface-container-high);
  color: var(--matui-on-surface);
  border-radius: 28px;
  padding: 24px;
  max-width: 360px;
  font-family: var(--matui-font);
  box-shadow: 0 8px 12px 6px rgba(0, 0, 0, 0.15);
}
.matui-dialog__title { margin: 0 0 16px; font-size: 1.5rem; line-height: 2rem; font-weight: 500; }
.matui-dialog__body { font-size: 0.875rem; line-height: 1.25rem; color: var(--matui-on-surface-variant); margin-bottom: 24px; }
.matui-dialog__actions { display: flex; justify-content: flex-end; gap: 8px; }

.matui-snackbar {
  position: fixed;
  left: 16px;
  right: 16px;
  bottom: 16px;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 16px;
  border-radius: 8px;
  background: var(--matui-inverse-surface);
  color: var(--matui-inverse-on-surface);
  font-family: var(--matui-font);
  font-size: 0.875rem;
  box-shadow: 0 4px 8px 3px rgba(0, 0, 0, 0.15);
  z-index: 8;
}
.matui-snackbar--hidden { display: none; }
.matui-snackbar__action { margin-left: auto; background: none; border: none; color: var(--matui-inverse-primary); font-weight: 600; cursor: pointer; font-size: 0.875rem; }

.matui-bottom-sheet {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  background: var(--matui-surface-container-low);
  color: var(--matui-on-surface);
  border-radius: 28px 28px 0 0;
  padding: 12px 24px 32px;
  font-family: var(--matui-font);
  box-shadow: 0 -4px 8px 3px rgba(0, 0, 0, 0.15);
  z-index: 7;
}
.matui-bottom-sheet--hidden { display: none; }
.matui-bottom-sheet__handle { width: 32px; height: 4px; border-radius: 100px; background: var(--matui-outline-variant); margin: 0 auto 16px; }

.matui-navbar {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  justify-content: space-around;
  padding: 12px 8px 16px;
  background: var(--matui-surface-container);
  font-family: var(--matui-font);
  z-index: 5;
}
.matui-navbar__item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  background: none;
  border: none;
  cursor: pointer;
  color: var(--matui-on-surface-variant);
  font-size: 0.75rem;
  padding: 0 12px;
}
.matui-navbar__item-icon { padding: 4px 20px; border-radius: 100px; font-size: 1.375rem; line-height: 1.5rem; transition: background 0.15s ease; }
.matui-navbar__item--active { color: var(--matui-on-surface); font-weight: 600; }
.matui-navbar__item--active .matui-navbar__item-icon { background: var(--matui-secondary-container); color: var(--matui-on-secondary-container); }

.matui-rail {
  position: fixed;
  top: 0;
  bottom: 0;
  left: 0;
  width: 80px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px 0;
  background: var(--matui-surface-container-low);
  font-family: var(--matui-font);
  z-index: 5;
}
.matui-rail__header { font-size: 1.125rem; font-weight: 500; margin-bottom: 16px; }
.matui-rail__item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  background: none;
  border: none;
  cursor: pointer;
  color: var(--matui-on-surface-variant);
  font-size: 0.71875rem;
  width: 100%;
  padding: 0;
}
.matui-rail__item-icon { padding: 8px 24px; border-radius: 100px; font-size: 1.375rem; line-height: 1.5rem; transition: background 0.15s ease; }
.matui-rail__item--active { color: var(--matui-on-surface); font-weight: 600; }
.matui-rail__item--active .matui-rail__item-icon { background: var(--matui-secondary-container); color: var(--matui-on-secondary-container); }

.matui-drawer {
  position: fixed;
  top: 0;
  bottom: 0;
  left: 0;
  width: 280px;
  background: var(--matui-surface-container-low);
  color: var(--matui-on-surface);
  font-family: var(--matui-font);
  padding: 16px 12px;
  z-index: 6;
  box-shadow: 0 2px 6px 2px rgba(0, 0, 0, 0.1);
}
.matui-drawer--closed { display: none; }
.matui-drawer__header { padding: 16px; font-size: 1.125rem; font-weight: 500; }
.matui-drawer__item {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
  padding: 12px 16px;
  border: none;
  border-radius: 100px;
  background: none;
  cursor: pointer;
  color: var(--matui-on-surface-variant);
  font-size: 0.875rem;
  font-weight: 500;
  text-align: left;
}
.matui-drawer__item:hover { background: color-mix(in srgb, var(--matui-on-surface) 8%, transparent); }
.matui-drawer__item--active { background: var(--matui-secondary-container); color: var(--matui-on-secondary-container); }

.matui-search {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
  max-width: 480px;
  padding: 12px 16px;
  border-radius: 100px;
  background: var(--matui-surface-container-high);
  color: var(--matui-on-surface-variant);
  font-family: var(--matui-font);
}
.matui-search input {
  flex: 1;
  border: none;
  background: transparent;
  outline: none;
  font: inherit;
  font-size: 0.95rem;
  color: var(--matui-on-surface);
}

.matui-tooltip { position: relative; display: inline-flex; cursor: default; }
.matui-tooltip__bubble {
  visibility: hidden;
  position: absolute;
  bottom: calc(100% + 8px);
  left: 50%;
  transform: translateX(-50%);
  padding: 6px 10px;
  border-radius: 4px;
  background: var(--matui-inverse-surface);
  color: var(--matui-inverse-on-surface);
  font-family: var(--matui-font);
  font-size: 0.75rem;
  white-space: nowrap;
  z-index: 9;
}
.matui-tooltip:hover .matui-tooltip__bubble { visibility: visible; }

.matui-menu {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  min-width: 180px;
  background: var(--matui-surface-container-high);
  border-radius: 8px;
  padding: 8px 0;
  font-family: var(--matui-font);
  box-shadow: 0 4px 8px 3px rgba(0, 0, 0, 0.15);
  z-index: 9;
}
.matui-menu--hidden { display: none; }
.matui-menu__item {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
  padding: 10px 16px;
  border: none;
  background: none;
  cursor: pointer;
  color: var(--matui-on-surface);
  font-size: 0.875rem;
  text-align: left;
}
.matui-menu__item:hover { background: color-mix(in srgb, var(--matui-on-surface) 8%, transparent); }

.matui-banner {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  padding: 16px;
  border-radius: 16px;
  background: var(--matui-surface-container);
  color: var(--matui-on-surface);
  font-family: var(--matui-font);
}
.matui-banner__icon { font-size: 1.5rem; color: var(--matui-on-surface-variant); }
.matui-banner__text { flex: 1; font-size: 0.875rem; line-height: 1.25rem; }
.matui-banner__actions { display: flex; gap: 8px; margin-top: 8px; }

.matui-carousel {
  display: flex;
  gap: 12px;
  overflow-x: auto;
  scroll-snap-type: x mandatory;
  padding-bottom: 8px;
}
.matui-carousel__item { scroll-snap-align: start; flex: 0 0 auto; }
"#,
    )
}
