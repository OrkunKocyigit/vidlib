import { Group, Rating, Text } from '@mantine/core';
import { type VideoFile } from '../../../entities/VideoFile';
import { useEffect, useState } from 'react';
import { SetRating } from '../../../service/SetRating';

interface VideoRatingProps {
  video?: VideoFile;
}

function VideoRating(props: VideoRatingProps): JSX.Element {
  const [rating, setRating] = useState(props.video?.video?.rating);

  useEffect(() => {
    if (props.video?.video != null) {
      setRating(props.video.video.rating);
    }
  }, [props.video?.video]);

  function updateVideoRating(video: VideoFile, newRating: number): void {
    SetRating(video, newRating)
      .then((value) => {
        setRating(value.response);
      })
      .catch((reason) => {
        console.error(reason);
      });
  }

  return (
    <Group>
      {props.video?.video != null ? (
        <>
          <Text>Rating: </Text>
          <Rating value={rating} onChange={updateVideoRating.bind(null, props.video)}></Rating>
        </>
      ) : null}
    </Group>
  );
}

export default VideoRating;
