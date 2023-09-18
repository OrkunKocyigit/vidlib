import { type VideoFile } from '../../../entities/VideoFile';
import { useEffect, useState } from 'react';
import { type VideoMediaInfo } from '../../../entities/VideoMediaInfo';
import { GetMediaInfo, type VideoMediaInfoEmitEvent } from '../../../service/GetMediaInfo';
import { Grid, Paper, Text } from '@mantine/core';
import useStyles from './VideoMetadataView.styles';
import { useTranslation } from 'react-i18next';
import { listen } from '@tauri-apps/api/event';

export interface VideoMediaInfoProps {
  video: VideoFile;
}

export function gridFields(): Array<keyof VideoMediaInfo> {
  return [
    'width',
    'height',
    'framerate',
    'length',
    'filesize',
    'bitrate',
    'codec',
    'acodec',
    'abitrate',
    'asample'
  ];
}
function VideoMetadataView(props: VideoMediaInfoProps): JSX.Element {
  const [metadata, setMetadata] = useState<VideoMediaInfo | undefined>(undefined);
  const { classes } = useStyles();
  useEffect(() => {
    setMetadata(undefined);
    GetMediaInfo(props.video.id, props.video.path).catch((reason) => {
      console.error(reason);
    });
    const eventName = `update_mediainfo_${props.video.id}`;
    const unlisten = listen<VideoMediaInfoEmitEvent>(eventName, (event) => {
      setMetadata(event.payload.media_info);
    });
    return () => {
      unlisten
        .then((value) => {
          value();
        })
        .catch((reason) => {
          console.error(reason);
        });
    };
  }, [props.video]);
  const { t } = useTranslation();

  return (
    <Grid gutter={'sm'}>
      {metadata !== undefined
        ? gridFields().map((key, index) => (
            <Grid.Col key={key} sm={12} md={6}>
              <Paper shadow={'sm'} radius={'md'} p={'sm'} withBorder>
                <Grid className={classes.data}>
                  <Grid.Col sm={7} className={classes.header}>
                    <Text tt={'capitalize'} fw={700}>
                      {t(key)}
                    </Text>
                  </Grid.Col>
                  <Grid.Col sm={5} className={classes.content}>
                    {metadata[key] != null ? metadata[key] : t('none')}
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
