import { Box, Image } from '@mantine/core';
import { type VideoFile } from '../../../entities/VideoFile';
import { useEffect, useState } from 'react';
import { GetThumbnail, type GetThumbnailEvent } from '../../../service/GetThumbnail';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import classes from './VideoThumbnail.module.pcss';

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
    let unlisten = new Promise<UnlistenFn>((resolve) => {
      resolve(() => {});
    });
    if (props.video != null) {
      GetThumbnail(props.video.id, props.video.path)
        .then((value) => {
          if (value.response !== undefined) {
            setImageSrc(value.response);
          }
        })
        .catch((reason) => {
          console.error(reason);
        });
      const eventName = `update_thumbnail_${props.video.id}`;
      unlisten = listen<GetThumbnailEvent>(eventName, (event) => {
        setImageSrc([convertFileSrc(event.payload.path)]);
      });
    }
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

  return (
    <Box mr="md" className={classes.box}>
      <Image
        w="10rem"
        h="10rem"
        mx="auto"
        fit="contain"
        radius="md"
        fallbackSrc="https://placehold.co/900x600?text=Thumbnail"
        src={imageUrl(imageSrc)}></Image>
    </Box>
  );
}

export default VideoThumbnail;
