import { cleanup, fireEvent, render, screen } from '@testing-library/svelte';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import Toast from './Toast.svelte';

describe('Toast Component', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.restoreAllMocks();
    cleanup();
  });

  it('rendert Toast mit Message', () => {
    render(Toast, { props: { message: 'Test Message' } });
    expect(screen.getByText('Test Message')).toBeInTheDocument();
  });

  it('rendert alle Typen korrekt', () => {
    cleanup();
    render(Toast, { props: { type: 'success', message: 'Test' } });
    expect(screen.getByRole('status')).toHaveClass('toast-success');
    // Icon ist in span.toast-icon mit aria-hidden
    expect(screen.getByText('✔️')).toBeInTheDocument();

    cleanup();
    render(Toast, { props: { type: 'error', message: 'Test' } });
    expect(screen.getByRole('status')).toHaveClass('toast-error');
    expect(screen.getByText('❌')).toBeInTheDocument();

    cleanup();
    render(Toast, { props: { type: 'warning', message: 'Test' } });
    expect(screen.getByRole('status')).toHaveClass('toast-warning');
    expect(screen.getByText('⚠️')).toBeInTheDocument();

    cleanup();
    render(Toast, { props: { type: 'info', message: 'Test' } });
    expect(screen.getByRole('status')).toHaveClass('toast-info');
    expect(screen.getByText('ℹ️')).toBeInTheDocument();
  });

  it('schließt automatisch nach Duration', async () => {
    const mockClose = vi.fn();
    render(Toast, { props: { message: 'Test', duration: 1000, onClose: mockClose } });

    expect(mockClose).not.toHaveBeenCalled();

    // Advance timers: 1000ms für auto-close + 200ms für Animation
    vi.advanceTimersByTime(1200);

    expect(mockClose).toHaveBeenCalledTimes(1);
  });

  it('schließt nicht automatisch wenn Duration 0', () => {
    const mockClose = vi.fn();
    render(Toast, { props: { message: 'Test', duration: 0, onClose: mockClose } });

    vi.advanceTimersByTime(5000);
    expect(mockClose).not.toHaveBeenCalled();
  });

  it('schließt bei Close-Button-Click', async () => {
    const mockClose = vi.fn();
    render(Toast, { props: { message: 'Test', onClose: mockClose } });

    const closeButton = screen.getByRole('button', { name: 'Schließen' });
    await fireEvent.click(closeButton);

    // Warte auf Animation (200ms)
    vi.advanceTimersByTime(200);

    expect(mockClose).toHaveBeenCalledTimes(1);
  });

  it('hat korrekte ARIA-Attribute', () => {
    render(Toast, { props: { message: 'Test Message', type: 'info' } });
    const toast = screen.getByRole('status');
    expect(toast).toHaveAttribute('aria-live', 'polite');

    // Icon span hat aria-hidden="true"
    const icon = toast.querySelector('.toast-icon');
    expect(icon).toHaveAttribute('aria-hidden', 'true');
  });

  // Slot-Test: Nicht unterstützt in @testing-library/svelte für Svelte 5
  // TODO: Implementiere mit Wrapper-Komponente wenn nötig

  it('wendet korrekte CSS-Klassen an', () => {
    render(Toast, { props: { type: 'success', message: 'Test' } });
    const toast = screen.getByRole('status');
    expect(toast).toHaveClass('toast', 'toast-success');
  });
});
