import React from "react";
import {FolderInfo} from "../../../entities/FolderInfo";
import {ActionIcon, Navbar} from "@mantine/core";
import {SearchBar} from "../../../components/SearchBar";
import {Plus} from "tabler-icons-react";
import FileTreeView from "./FileTreeView";

type Props = {
    folders: FolderInfo[]
}

type State = {

}
class SideBar extends React.Component<Props, State> {
    constructor(props: Props) {
        super(props);
    }

    render() {
        return (
            <Navbar width={{
                base: 300
            }}>
                <Navbar.Section>
                    <SearchBar>
                        <ActionIcon variant="default"><Plus size="1rem" /></ActionIcon>
                    </SearchBar>
                </Navbar.Section>
                <Navbar.Section grow>
                    <FileTreeView folders={this.props.folders}></FileTreeView>
                </Navbar.Section>
            </Navbar>
        )
    }
}

export default SideBar