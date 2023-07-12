import {FolderInfo} from "../../../entities/FolderInfo";
import React from "react";
import FileTreeView from "./FileTreeView";
import VideoFileView from "./VideoFileView";
import {ArrowDown, ArrowRight} from "tabler-icons-react";
import {Group, Text, ThemeIcon, UnstyledButton} from "@mantine/core";

type Props = {
    folder: FolderInfo
}

function renderFolderIcon(folder: FolderInfo, state: boolean) {
    if (folder.empty) {
        return null
    }
    return (
        <ThemeIcon>
            {state ?
                <ArrowDown></ArrowDown>
                :
                <ArrowRight></ArrowRight>
            }
        </ThemeIcon>
    )
}

function FolderInfoView(props: Props) {
    return (
        <div>
            <UnstyledButton>
                <Group>
                    {renderFolderIcon(props.folder, false)}
                    <Text>{props.folder.path}</Text>
                </Group>
            </UnstyledButton>
            <FileTreeView folders={props.folder.folders}></FileTreeView>
            {props.folder.videos.map(video => <VideoFileView video={video} key={video.path}/>)}
        </div>
    )
}

export default FolderInfoView