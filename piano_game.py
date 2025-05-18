import pygame
import mido
from mido import MidiInput
import sys

# Initialize pygame
pygame.init()

# Set up display
WIDTH, HEIGHT = 800, 600
screen = pygame.display.set_mode((WIDTH, HEIGHT))
pygame.display.set_caption('MIDI Piano Game')

# Set up font
font = pygame.font.SysFont(None, 36)

# --- Song Data: Faithless - Insomnia (Simplified Main Riff) ---
# MIDI note numbers for C4 = 60
# This is a simplified, iconic riff for learning purposes
# Format: (note, start_time_in_beats, duration_in_beats)
SONG_TEMPO_BPM = 120
SONG_BEAT_DURATION = 60 / SONG_TEMPO_BPM
SONG_NOTES = [
    # (MIDI note, start_beat, duration_beat)
    (64, 0, 1),   # E4
    (67, 1, 1),   # G4
    (69, 2, 1),   # A4
    (67, 3, 1),   # G4
    (64, 4, 1),   # E4
    (62, 5, 1),   # D4
    (60, 6, 2),   # C4 (hold)
    (62, 8, 1),   # D4
    (64, 9, 1),   # E4
    (67, 10, 1),  # G4
    (69, 11, 1),  # A4
    (67, 12, 1),  # G4
    (64, 13, 1),  # E4
    (62, 14, 1),  # D4
    (60, 15, 2),  # C4 (hold)
]

# MIDI setup
try:
    input_names = mido.get_input_names()
    if not input_names:
        print('No MIDI input devices found.')
        sys.exit(1)
    print('Available MIDI inputs:')
    for i, name in enumerate(input_names):
        print(f'{i}: {name}')
    midi_input = mido.open_input(input_names[0])
    print(f'Connected to: {input_names[0]}')
except Exception as e:
    print(f'Error connecting to MIDI device: {e}')
    sys.exit(1)

# Main game loop
running = True
clock = pygame.time.Clock()

while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    # Fill background
    screen.fill((30, 30, 30))

    # Display message
    text = font.render('MIDI Piano Game - Faithless "Insomnia" (Simplified)', True, (255, 255, 255))
    screen.blit(text, (50, HEIGHT // 2 - 20))

    pygame.display.flip()
    clock.tick(60)

pygame.quit() 