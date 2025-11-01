import { cleanup, fireEvent, render, screen } from '@testing-library/svelte';
import PortableNotice from './PortableNotice.svelte';
import { afterEach, describe, expect, it, vi } from 'vitest';

const baseStatus = {
  portable_dir: './config/portable',
  effective_dir: './config/portable',
  read_only: false,
  fallback_used: false,
  encrypted: false,
};

describe('PortableNotice', () => {
  afterEach(() => {
    cleanup();
  });

  it('zeigt Warnung wenn Fallback aktiv ist', () => {
    render(PortableNotice, {
      props: {
        status: {
          ...baseStatus,
          fallback_used: true,
          effective_dir: '/tmp/fallback',
        },
      },
    });

    expect(screen.getByText('Portabler Speicher im Fallback-Modus')).toBeInTheDocument();
    // Es gibt mehrere Elemente mit "fallback", nutze getAllByText
    const fallbackElements = screen.getAllByText(/fallback/i);
    expect(fallbackElements.length).toBeGreaterThan(0);
    expect(screen.getByText('Fallback aktiv')).toBeInTheDocument();
  });

  it('zeigt Hinweis auf Schreibschutz', () => {
    render(PortableNotice, {
      props: {
        status: {
          ...baseStatus,
          read_only: true,
        },
      },
    });

    expect(screen.getByText('Portabler Speicher schreibgeschützt')).toBeInTheDocument();
    expect(screen.getByText(/Schreibrechte/)).toBeInTheDocument();
  });

  it('emittiert dismiss Event über Close-Button', async () => {
    const handleDismiss = vi.fn();
    render(PortableNotice, {
      props: {
        status: {
          ...baseStatus,
          fallback_used: true,
        },
        ondismiss: handleDismiss,
      },
    });

    await fireEvent.click(screen.getByRole('button', { name: 'Hinweis schließen' }));
    expect(handleDismiss).toHaveBeenCalledTimes(1);
  });

  it('rendert Aktionen', async () => {
    const action = vi.fn();
    render(PortableNotice, {
      props: {
        status: {
          ...baseStatus,
          fallback_used: true,
        },
        actions: [
          {
            label: 'Anleitung öffnen',
            onClick: action,
          },
        ],
      },
    });

    const actionButton = screen.getByRole('button', { name: 'Anleitung öffnen' });
    await fireEvent.click(actionButton);
    expect(action).toHaveBeenCalledTimes(1);
  });

  it('rendert als Modal-Layout', () => {
    render(PortableNotice, {
      props: {
        status: baseStatus,
        layout: 'modal',
      },
    });

    expect(screen.getByRole('dialog')).toBeDefined();
    expect(screen.getByText('Portable Konfiguration aktiv')).toBeInTheDocument();
  });
});
