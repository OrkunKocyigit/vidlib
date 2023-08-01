import { type VideoFile } from '../../../entities/VideoFile';
import UpdatableText from '../../../components/UpdatableText';
import { SetName } from '../../../service/SetName';

export interface VideoNameProps {
  video: VideoFile;
}
function VideoName(props: VideoNameProps): JSX.Element {
  function setText(name: string): void {
    SetName(props.video, name)
      .then((value) => {
        const { response: name = '' } = value;
        props.video.name = name;
      })
      .catch((reason) => {
        console.error(reason);
      });
  }

  return (
    <UpdatableText
      text={props.video.name}
      setText={setText}
      titleProps={{ order: 4 }}
      editButtonProps={{ color: 'blue' }}
      saveButtonProps={{ color: 'green' }}
      cancelButtonProps={{ color: 'red' }}></UpdatableText>
  );
}

export default VideoName;
