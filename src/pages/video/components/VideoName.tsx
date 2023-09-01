import { type VideoFile } from '../../../entities/VideoFile';
import UpdatableText from '../../../components/UpdatableText';
import { SetName } from '../../../service/SetName';
import { useEffect, useState } from 'react';

export interface VideoNameProps {
  video: VideoFile;
}
function VideoName(props: VideoNameProps): JSX.Element {
  const [name, setName] = useState(props.video.video?.name);
  function setText(newName: string): void {
    SetName(props.video, newName)
      .then((value) => {
        const { response: responseName = '' } = value;
        setName(responseName);
      })
      .catch((reason) => {
        console.error(reason);
      });
  }

  useEffect(() => {
    setName(props.video.video?.name);
  }, [props.video]);

  return (
    <UpdatableText
      text={name ?? ''}
      setText={setText}
      titleProps={{ order: 4 }}
      editButtonProps={{ color: 'blue' }}
      saveButtonProps={{ color: 'green' }}
      cancelButtonProps={{ color: 'red' }}
    ></UpdatableText>
  );
}

export default VideoName;
