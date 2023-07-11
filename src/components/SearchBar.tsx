import {Box, Group, rem, TextInput, useMantineColorScheme} from "@mantine/core";
import {Search} from "tabler-icons-react";
import {ReactNode} from "react";

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
                <TextInput placeholder="Filename" icon={<Search size="1rem"></Search>}></TextInput>
                {children}
            </Group>
        </Box>
    )
}