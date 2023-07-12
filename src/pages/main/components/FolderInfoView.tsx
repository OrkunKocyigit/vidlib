import {FolderInfo} from "../../../entities/FolderInfo";
import React from "react";
import FileTreeView from "./FileTreeView";
import VideoFileView from "./VideoFileView";

type Props = {
    folder: FolderInfo
}

function FolderInfoView(props: Props) {
    return (
        <div>
            {JSON.stringify(props.folder, null, 4)}
            <FileTreeView folders={props.folder.folders}></FileTreeView>
            {props.folder.videos.map(video => <VideoFileView video={video} key={video.path}/>)}
        </div>
    )
}

export default FolderInfoView