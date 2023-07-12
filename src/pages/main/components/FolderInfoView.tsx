import {FolderInfo} from "../../../entities/FolderInfo";
import React from "react";
import FileTreeView from "./FileTreeView";
import VideoFileView from "./VideoFileView";
import {Group, Text, UnstyledButton} from "@mantine/core";
import {IconArrowDown, IconArrowRight, IconFolder, IconFolderOpen} from "@tabler/icons-react";

type Props = {
    folder: FolderInfo
}

function renderToggleIcon(folder: FolderInfo, state: boolean) {
    if (folder.empty) {
        return null
    } else if (state) {
        return <IconArrowDown></IconArrowDown>
    } else {
        return <IconArrowRight></IconArrowRight>
    }
}

function renderFolderIcon(folder: FolderInfo, state: boolean) {
    if (folder.empty || state) {
        return <IconFolderOpen></IconFolderOpen>
    } else {
        return <IconFolder></IconFolder>
    }
}

function FolderInfoView(props: Props) {
    return (
        <div>
            <UnstyledButton>
                <Group>
                    {renderToggleIcon(props.folder, false)}
                    {renderFolderIcon(props.folder, false)}
                    <Text>{props.folder.name}</Text>
                </Group>
            </UnstyledButton>
            <FileTreeView folders={props.folder.folders}></FileTreeView>
            {props.folder.videos.map(video => <VideoFileView video={video} key={video.path}/>)}
        </div>
    )
}

export default FolderInfoView