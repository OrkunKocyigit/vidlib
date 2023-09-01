import { Box, Image, Text } from '@mantine/core';
import { type VideoFile } from '../../../entities/VideoFile';
import { useEffect, useState } from 'react';
import { GetThumbnail } from '../../../service/GetThumbnail';

export interface VideoThumbnailProps {
  video?: VideoFile;
}

function VideoThumbnail(props: VideoThumbnailProps): JSX.Element {
  const [imageSrc, setImageSrc] = useState<string[] | undefined>(undefined);

  function imageUrl(imageSrc: string[] | undefined): string {
    if (imageSrc != null) {
      return imageSrc[0];
    }
    return '';
  }

  useEffect(() => {
    setImageSrc(undefined);
    if (props.video != null) {
      GetThumbnail(props.video)
        .then((value) => {
          setImageSrc(value.response as string[]);
        })
        .catch((reason) => {
          console.error(reason);
        });
    }
  }, [props.video]);

  return (
    <Box
      mr={'md'}
      sx={(theme) => ({
        backgroundColor: theme.colorScheme === 'dark' ? 'white' : 'black'
      })}
    >
      <Image
        width={'10rem'}
        height={'10rem'}
        mx={'auto'}
        fit={'contain'}
        radius={'md'}
        placeholder={<Text align={'center'}>Thumbnail</Text>}
        withPlaceholder
        src={imageUrl(imageSrc)}
      ></Image>
    </Box>
  );
}

export default VideoThumbnail;
