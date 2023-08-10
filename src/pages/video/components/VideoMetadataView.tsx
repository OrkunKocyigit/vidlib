import { type VideoFile } from '../../../entities/VideoFile';
import { useEffect, useState } from 'react';
import { type VideoMetadata } from '../../../entities/VideoMetadata';
import { GetMetadata } from '../../../service/GetMetadata';
import { Grid, Paper, Text } from '@mantine/core';
import useStyles from './VideoMetadataView.styles';
import { useTranslation } from 'react-i18next';

export interface VideoMetadataProps {
  video: VideoFile;
}
function VideoMetadataView(props: VideoMetadataProps): JSX.Element {
  const [metadata, setMetadata] = useState<VideoMetadata | undefined>(undefined);
  const { classes } = useStyles();
  useEffect(() => {
    setMetadata(undefined);
    GetMetadata(props.video)
      .then((value) => {
        setMetadata(value.response);
      })
      .catch((reason) => {
        console.error(reason);
      });
  }, [props.video]);
  const { t } = useTranslation();

  return (
    <Grid gutter={'sm'}>
      {metadata !== undefined
        ? Object.entries(metadata).map(([key, value], index) => (
            <Grid.Col key={key} sm={12} md={6}>
              <Paper shadow={'sm'} radius={'md'} p={'sm'} withBorder>
                <Grid className={classes.data}>
                  <Grid.Col sm={8} className={classes.header}>
                    <Text tt={'capitalize'} fw={700}>
                      {t(key)}
                    </Text>
                  </Grid.Col>
                  <Grid.Col sm={4} className={classes.content}>
                    {value != null ? value : t('none')}
                  </Grid.Col>
                </Grid>
              </Paper>
            </Grid.Col>
          ))
        : null}
    </Grid>
  );
}

export default VideoMetadataView;
