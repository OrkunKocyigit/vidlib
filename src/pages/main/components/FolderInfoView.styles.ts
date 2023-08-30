import { createStyles, rem } from '@mantine/core';

export type FolderInfoViewVariants = 'watched' | undefined;

export interface FolderInfoViewStyleProps {
  variant: FolderInfoViewVariants;
}
export default createStyles((theme, { variant }: FolderInfoViewStyleProps) => ({
  control: {
    fontWeight: 500,
    display: 'block',
    width: '100%',
    padding: `${theme.spacing.xs} ${theme.spacing.md}`,
    color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.black,
    backgroundColor:
      variant === 'watched'
        ? theme.colors.green[0]
        : theme.colorScheme === 'light'
        ? 'white'
        : theme.colors.dark[0],
    fontSize: theme.fontSizes.sm,
    whiteSpace: 'nowrap',

    '&:hover': {
      backgroundColor:
        variant === 'watched'
          ? theme.colors.green[4]
          : theme.colorScheme === 'dark'
          ? theme.colors.dark[7]
          : theme.colors.gray[0],
      color: theme.colorScheme === 'dark' || variant === 'watched' ? theme.white : theme.black
    }
  },

  link: {
    fontWeight: 500,
    display: 'block',
    paddingLeft: rem(4),
    marginLeft: rem(24),
    fontSize: theme.fontSizes.sm,
    color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.colors.gray[7],
    borderLeft: `${rem(1)} solid ${
      theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[3]
    }`
  },

  icon: {
    transition: 'transform 200ms ease'
  }
}));
