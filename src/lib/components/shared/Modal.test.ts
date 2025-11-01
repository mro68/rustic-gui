import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import Modal from './Modal.svelte';
import ModalTestWrapper from './ModalTestWrapper.svelte';

describe('Modal Component', () => {
  beforeEach(() => {
    // Reset body overflow
    document.body.style.overflow = '';
  });

  afterEach(() => {
    cleanup();
  });

  it('rendert nicht wenn nicht offen', () => {
    render(Modal, { props: { open: false } });
    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
  });

  it('rendert Modal wenn offen', () => {
    render(Modal, { props: { open: true } });
    expect(screen.getByRole('dialog')).toBeInTheDocument();
    expect(screen.getByRole('presentation')).toHaveClass('modal-backdrop');
  });

  it('schließt bei ESC-Taste', async () => {
    const mockClose = vi.fn();
    render(Modal, {
      props: { open: true, onclose: mockClose },
    });

    await fireEvent.keyDown(document, { key: 'Escape' });

    // Warte auf Animation-Timeout (180ms)
    await waitFor(() => expect(mockClose).toHaveBeenCalledTimes(1), { timeout: 300 });
  });

  it('schließt nicht bei ESC wenn closeOnEsc false', async () => {
    const mockClose = vi.fn();
    render(Modal, {
      props: { open: true, closeOnEsc: false, onclose: mockClose },
    });

    await fireEvent.keyDown(document, { key: 'Escape' });
    expect(mockClose).not.toHaveBeenCalled();
  });

  it('schließt bei Backdrop-Click', async () => {
    const mockClose = vi.fn();
    render(Modal, {
      props: { open: true, onclose: mockClose },
    });

    const backdrop = screen.getByRole('presentation');
    await fireEvent.click(backdrop);

    // Warte auf Animation-Timeout (180ms)
    await waitFor(() => expect(mockClose).toHaveBeenCalledTimes(1), { timeout: 300 });
  });

  it('schließt nicht bei Backdrop-Click wenn closeOnBackdrop false', async () => {
    const mockClose = vi.fn();
    render(Modal, {
      props: { open: true, closeOnBackdrop: false, onclose: mockClose },
    });

    const backdrop = screen.getByRole('presentation');
    await fireEvent.click(backdrop);
    expect(mockClose).not.toHaveBeenCalled();
  });

  it('schließt bei Close-Button-Click', async () => {
    const mockClose = vi.fn();
    render(ModalTestWrapper, {
      props: { open: true, onclose: mockClose, showHeader: true },
    });

    const closeButton = screen.getByRole('button', { name: 'Schließen' });
    await fireEvent.click(closeButton);

    // Warte auf Animation-Timeout (180ms)
    await waitFor(() => expect(mockClose).toHaveBeenCalledTimes(1), { timeout: 300 });
  });
  it('hat korrekte ARIA-Attribute', () => {
    render(Modal, { props: { open: true, ariaLabel: 'Test Modal' } });
    const dialog = screen.getByRole('dialog');
    expect(dialog).toHaveAttribute('aria-modal', 'true');
    expect(dialog).toHaveAttribute('aria-label', 'Test Modal');
  });

  // Slot-Tests: Nicht unterstützt in @testing-library/svelte für Svelte 5
  // TODO: Implementiere mit Wrapper-Komponenten wenn nötig

  it('blockiert Body-Scroll wenn offen', () => {
    render(Modal, { props: { open: true } });
    expect(document.body.style.overflow).toBe('hidden');
  });

  it('erlaubt Body-Scroll wenn geschlossen', async () => {
    const { rerender } = render(Modal, { props: { open: true } });
    expect(document.body.style.overflow).toBe('hidden');

    rerender({ open: false });

    // Warte kurz damit reactive statement ausgeführt wird
    await waitFor(() => expect(document.body.style.overflow).toBe(''));
  });

  it('fokussiert Dialog beim Öffnen', async () => {
    render(Modal, { props: { open: true } });
    const dialog = screen.getByRole('dialog');

    // Warte auf Focus-Timeout
    await new Promise((resolve) => setTimeout(resolve, 10));
    expect(document.activeElement).toBe(dialog);
  });
});
