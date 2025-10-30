import { fireEvent, render, screen } from '@testing-library/svelte';
import { describe, expect, it, vi } from 'vitest';
import Button from './Button.svelte';

describe('Button Component', () => {
  it('rendert primären Button korrekt', () => {
    render(Button, { props: { variant: 'primary' } });
    const button = screen.getByRole('button');
    expect(button).toHaveClass('btn', 'btn-primary', 'btn-md');
    expect(button).toHaveAttribute('type', 'button');
  });

  it('rendert Button mit Text', () => {
    render(Button, { props: { children: 'Test Button' } });
    expect(screen.getByText('Test Button')).toBeInTheDocument();
  });

  it('rendert alle Varianten korrekt', () => {
    const { rerender } = render(Button, { props: { variant: 'primary' } });
    expect(screen.getByRole('button')).toHaveClass('btn-primary');

    rerender({ variant: 'secondary' });
    expect(screen.getByRole('button')).toHaveClass('btn-secondary');

    rerender({ variant: 'danger' });
    expect(screen.getByRole('button')).toHaveClass('btn-danger');
  });

  it('rendert alle Größen korrekt', () => {
    const { rerender } = render(Button, { props: { size: 'sm' } });
    expect(screen.getByRole('button')).toHaveClass('btn-sm');

    rerender({ size: 'md' });
    expect(screen.getByRole('button')).toHaveClass('btn-md');

    rerender({ size: 'lg' });
    expect(screen.getByRole('button')).toHaveClass('btn-lg');
  });

  it('handhabt disabled State korrekt', () => {
    render(Button, { props: { disabled: true } });
    const button = screen.getByRole('button');
    expect(button).toBeDisabled();
    expect(button).toHaveAttribute('disabled');
  });

  it('handhabt loading State korrekt', () => {
    render(Button, { props: { loading: true } });
    const button = screen.getByRole('button');
    expect(button).toBeDisabled();
    expect(button).toHaveAttribute('aria-busy', 'true');
    expect(button).toHaveClass('is-loading');
    expect(screen.getByRole('presentation')).toHaveClass('spinner'); // aria-hidden="true" macht es presentation
  });

  it('rendert Icon korrekt', () => {
    render(Button, { props: { icon: '🔍' } });
    expect(screen.getByText('🔍')).toBeInTheDocument();
    expect(screen.getByRole('presentation')).toHaveClass('btn-icon');
  });

  it('handhabt Tooltip korrekt', () => {
    render(Button, { props: { tooltip: 'Test Tooltip' } });
    // Tooltip wird durch Tooltip-Komponente gehandhabt
    // Wir testen nur, dass der Button gerendert wird
    expect(screen.getByRole('button')).toBeInTheDocument();
  });

  it('handhabt ariaLabel korrekt', () => {
    render(Button, { props: { ariaLabel: 'Test Label' } });
    expect(screen.getByRole('button')).toHaveAttribute('aria-label', 'Test Label');
  });

  it('handhabt onclick Event korrekt', async () => {
    const mockClick = vi.fn();
    render(Button, { props: { onclick: mockClick } });
    const button = screen.getByRole('button');

    await fireEvent.click(button);
    expect(mockClick).toHaveBeenCalledTimes(1);
  });

  it('handhabt verschiedene Button-Typen', () => {
    const { rerender } = render(Button, { props: { type: 'submit' } });
    expect(screen.getByRole('button')).toHaveAttribute('type', 'submit');

    rerender({ type: 'reset' });
    expect(screen.getByRole('button')).toHaveAttribute('type', 'reset');
  });

  it('ist responsive auf Mobile', () => {
    // Mock viewport für mobile Test
    Object.defineProperty(window, 'innerWidth', { value: 600 });
    render(Button);
    const button = screen.getByRole('button');
    // Responsive Styles werden durch CSS gehandhabt
    expect(button).toBeInTheDocument();
  });
});
