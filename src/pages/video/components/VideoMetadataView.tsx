import { type VideoFile } from '../../../entities/VideoFile';
import { useEffect, useState } from 'react';
import { type VideoMediaInfo } from '../../../entities/VideoMediaInfo';
import { GetMediaInfo, type VideoMediaInfoEmitEvent } from '../../../service/GetMediaInfo';
import { Grid, Paper, Text } from '@mantine/core';
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
    <Grid gutter="sm">
      {metadata !== undefined
        ? gridFields().map((key, index) => (
            <Grid.Col
              key={key}
              span={{
                sm: 12,
                md: 6
              }}>
              <Paper shadow="sm" radius="md" p="sm" withBorder>
                <Grid>
                  <Grid.Col span={{ sm: 7 }}>
                    <Text tt="capitalize" fw={700}>
                      {t(key)}
                    </Text>
                  </Grid.Col>
                  <Grid.Col span={{ sm: 5 }}>{metadata[key] ?? t('none')}</Grid.Col>
                </Grid>
              </Paper>
            </Grid.Col>
          ))
        : null}
    </Grid>
  );
}

export default VideoMetadataView;
