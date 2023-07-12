import {Box, Group, rem, TextInput} from "@mantine/core";
import {ReactNode} from "react";
import {IconSearch} from "@tabler/icons-react";

type Props = {
    children: ReactNode
}
export function SearchBar({children}: Props) {
    return (
        <Box sx={(theme) => ({
            paddingTop: theme.spacing.xs,
            paddingLeft: theme.spacing.xs,
            paddingRight: theme.spacing.xs,
            paddingBottom: theme.spacing.md,
            borderBottom: `${rem(1)} solid ${
                theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[2]
            }`
        })}>
            <Group>
                <TextInput placeholder="Filename" icon={<IconSearch size="1rem"></IconSearch>}></TextInput>
                {children}
            </Group>
        </Box>
    )
}