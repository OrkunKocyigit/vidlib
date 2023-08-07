import { type VideoFile } from '../../../entities/VideoFile';
import { useEffect, useState } from 'react';
import { type VideoMetadata } from '../../../entities/VideoMetadata';
import { GetMetadata } from '../../../service/GetMetadata';

export interface VideoMetadataProps {
  video: VideoFile;
}
function VideoMetadataView(props: VideoMetadataProps): JSX.Element {
  const [metadata, setMetadata] = useState<VideoMetadata | undefined>(undefined);
  useEffect(() => {
    GetMetadata(props.video)
      .then((value) => {
        setMetadata(value.response);
      })
      .catch((reason) => {
        console.error(reason);
      });
  }, [props.video]);
  return <div>{JSON.stringify(metadata, null, 4)}</div>;
}

export default VideoMetadataView;
