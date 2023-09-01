import { type VideoFile } from '../../../entities/VideoFile';
import { useEffect, useState } from 'react';
import { ActionIcon, Box, Group, Text } from '@mantine/core';
import { IconCheck, IconX } from '@tabler/icons-react';
import { SetWatched } from '../../../service/SetWatched';
import { useTranslation } from 'react-i18next';

interface VideoWatchProps {
  video: VideoFile;
}

function VideoWatch(props: VideoWatchProps): JSX.Element {
  const [watched, setWatched] = useState(props.video?.video?.watched);
  const { t } = useTranslation();

  useEffect(() => {
    if (props.video?.video != null) {
      setWatched(props.video.video.watched);
    }
  }, [props.video?.video]);

  function updateWatched(video: VideoFile, value: boolean): void {
    if (video.video != null) {
      SetWatched(video, value)
        .then((newWatched) => {
          setWatched(newWatched.response);
        })
        .catch((reason) => {
          console.error(reason);
        });
    }
  }

  return (
    <Box>
      {watched === true ? (
        <Group align={'center'}>
          <ActionIcon
            color={'green'}
            onClick={() => {
              updateWatched(props.video, false);
            }}
          >
            <IconCheck></IconCheck>
          </ActionIcon>
          <Text>{t('video.watch.watched')}</Text>
        </Group>
      ) : (
        <Group>
          <ActionIcon
            color={'red'}
            onClick={() => {
              updateWatched(props.video, true);
            }}
          >
            <IconX></IconX>
          </ActionIcon>
          <Text>{t('video.watch.not.watched')}</Text>
        </Group>
      )}
    </Box>
  );
}

export default VideoWatch;
