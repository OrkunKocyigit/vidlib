import { createStyles, type MantineTheme } from '@mantine/core';
import { type FolderInfoViewVariants } from './FolderInfoView.styles';

export type VideoFileViewVariants = 'selected' | 'selectedwatched' | FolderInfoViewVariants;

export interface VideoFileViewStyleProps {
  variant: VideoFileViewVariants;
}

function getBackgroundColor(variant: VideoFileViewVariants, theme: MantineTheme): string {
  if (variant === 'watched') {
    return theme.colors.green[0];
  } else if (variant === 'selectedwatched') {
    return theme.colors.green[3];
  } else if (theme.colorScheme === 'light') {
    if (variant === 'selected') {
      return theme.colors.gray[1];
    } else {
      return 'white';
    }
  } else {
    return theme.colors.dark[0];
  }
}

function getHoverBackgroundColor(variant: VideoFileViewVariants, theme: MantineTheme): string {
  if (variant === 'watched' || variant === 'selectedwatched') {
    return theme.colors.green[5];
  } else if (theme.colorScheme === 'dark') {
    return theme.colors.dark[7];
  } else {
    return theme.colors.gray[2];
  }
}

export default createStyles((theme, { variant }: VideoFileViewStyleProps) => ({
  control: {
    fontWeight: 500,
    display: 'block',
    width: '100%',
    padding: `${theme.spacing.xs} ${theme.spacing.md}`,
    color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.black,
    backgroundColor: getBackgroundColor(variant, theme),
    fontSize: theme.fontSizes.sm,
    whiteSpace: 'nowrap',

    '&:hover': {
      backgroundColor: getHoverBackgroundColor(variant, theme),
      color: theme.colorScheme === 'dark' || variant === 'watched' ? theme.white : theme.black
    }
  },

  icon: {
    transition: 'transform 200ms ease'
  }
}));
