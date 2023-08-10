import { createStyles } from '@mantine/core';

export default createStyles((theme) => ({
  data: {},
  header: {
    backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[1] : theme.colors.gray[0]
  },
  content: {}
}));
