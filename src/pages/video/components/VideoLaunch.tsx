import { type VideoFile } from '../../../entities/VideoFile';
import { ActionIcon, Group, Text } from '@mantine/core';
import { IconPlayerPlay } from '@tabler/icons-react';
import { OpenVideo } from '../../../service/OpenVideo';
import { useTranslation } from 'react-i18next';

export interface VideoLaunchProps {
  video: VideoFile;
}

function VideoLaunch(props: VideoLaunchProps): JSX.Element {
  const { t } = useTranslation();
  function openVideo(video: VideoFile): void {
    OpenVideo(video).catch((reason) => {
      console.error(reason);
    });
  }

  return (
    <Group align="center">
      <ActionIcon color="blue" variant="subtle" onClick={openVideo.bind(null, props.video)}>
        <IconPlayerPlay></IconPlayerPlay>
      </ActionIcon>
      <Text>{t('video.launch.play')}</Text>
    </Group>
  );
}

export default VideoLaunch;
