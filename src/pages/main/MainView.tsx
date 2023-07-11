import React from "react";
import {FolderInfo} from '../../entities/FolderInfo'
import {AppShell} from "@mantine/core";
import SideBar from "./components/SideBar";

type Props = {

}

type State = {
    folders: FolderInfo[]
}

class MainView extends React.Component<Props, State> {
    constructor(props: Props) {
        super(props)
        this.state = {
            folders: []
        }
    }

    render() {
        return (
            <AppShell padding="md" navbar={<SideBar folders={this.state.folders}></SideBar>}>
                Video View
            </AppShell>
        );
    }
}

export default MainView