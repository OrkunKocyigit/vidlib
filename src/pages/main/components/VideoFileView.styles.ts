import { createStyles } from '@mantine/core';

export type VideoFileViewVariants = 'watched' | undefined;

export interface VideoFileViewStyleProps {
  variant: VideoFileViewVariants;
}

export default createStyles((theme, { variant }: VideoFileViewStyleProps) => ({
  control: {
    fontWeight: 500,
    display: 'block',
    width: '100%',
    padding: `${theme.spacing.xs} ${theme.spacing.md}`,
    color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.black,
    backgroundColor:
      variant === 'watched'
        ? theme.colors.green[1]
        : theme.colorScheme === 'light'
        ? 'white'
        : theme.colors.dark[0],
    fontSize: theme.fontSizes.sm,
    whiteSpace: 'nowrap',

    '&:hover': {
      backgroundColor:
        variant === 'watched'
          ? theme.colors.green[7]
          : theme.colorScheme === 'dark'
          ? theme.colors.dark[7]
          : theme.colors.gray[0],
      color: theme.colorScheme === 'dark' || variant === 'watched' ? theme.white : theme.black
    }
  },

  icon: {
    transition: 'transform 200ms ease'
  }
}));
