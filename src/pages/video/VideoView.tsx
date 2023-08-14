import { type VideoFile } from '../../entities/VideoFile';
import { useEffect, useState } from 'react';
import { Flex, LoadingOverlay } from '@mantine/core';
import VideoHeader from './components/VideoHeader';
import { useDisclosure } from '@mantine/hooks';
import { GetVideo } from '../../service/GetVideo';
import VideoMetadata from './components/VideoMetadataView';
import VideoNotes from './components/VideoNotes';

export interface VideoViewProps {
  video?: VideoFile;
}

const VideoView = function (props: VideoViewProps): JSX.Element {
  const [visible, { open, close }] = useDisclosure(true);
  const [videoFile, setVideoFile] = useState<VideoFile | undefined>(undefined);

  useEffect(() => {
    if (props.video != null && (videoFile == null || videoFile.path !== props.video.path)) {
      open();
      GetVideo(props.video)
        .then((value) => {
          setVideoFile(value.response);
          close();
        })
        .catch((reason) => {
          console.error(reason);
        });
    }
  }, [props.video]);

  return (
    <Flex direction={'column'} gap={'md'} pos={'relative'} h={'100%'}>
      <LoadingOverlay visible={visible} overlayBlur={2}></LoadingOverlay>
      {videoFile != null ? (
        <>
          <VideoHeader video={videoFile}></VideoHeader>
          <VideoMetadata video={videoFile}></VideoMetadata>
          <VideoNotes video={videoFile}></VideoNotes>
        </>
      ) : null}
    </Flex>
  );
};

export default VideoView;
