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
    const { rerender } = render(Toast, { props: { type: 'success', message: 'Test' } });
    expect(screen.getByRole('status')).toHaveClass('toast-success');
    expect(screen.getByText('✔️')).toBeInTheDocument();

    rerender({ type: 'error', message: 'Test' });
    expect(screen.getByRole('status')).toHaveClass('toast-error');
    expect(screen.getByText('❌')).toBeInTheDocument();

    rerender({ type: 'warning', message: 'Test' });
    expect(screen.getByRole('status')).toHaveClass('toast-warning');
    expect(screen.getByText('⚠️')).toBeInTheDocument();

    rerender({ type: 'info', message: 'Test' });
    expect(screen.getByRole('status')).toHaveClass('toast-info');
    expect(screen.getByText('ℹ️')).toBeInTheDocument();
  });

  it('schließt automatisch nach Duration', async () => {
    const mockClose = vi.fn();
    render(Toast, { props: { message: 'Test', duration: 1000, onClose: mockClose } });

    expect(mockClose).not.toHaveBeenCalled();
    vi.advanceTimersByTime(1000);
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
    expect(mockClose).toHaveBeenCalledTimes(1);
  });

  it('hat korrekte ARIA-Attribute', () => {
    render(Toast, { props: { message: 'Test Message' } });
    const toast = screen.getByRole('status');
    expect(toast).toHaveAttribute('aria-live', 'polite');
    expect(screen.getByText('✔️')).toHaveAttribute('aria-hidden', 'true');
  });

  it('rendert Slot-Inhalt anstatt Message', () => {
    render(Toast, {
      props: { message: 'Default Message' },
      slots: {
        default: 'Custom Slot Content',
      },
    });
    expect(screen.getByText('Custom Slot Content')).toBeInTheDocument();
    expect(screen.queryByText('Default Message')).not.toBeInTheDocument();
  });

  it('wendet korrekte CSS-Klassen an', () => {
    render(Toast, { props: { type: 'success', message: 'Test' } });
    const toast = screen.getByRole('status');
    expect(toast).toHaveClass('toast', 'toast-success');
  });
});
