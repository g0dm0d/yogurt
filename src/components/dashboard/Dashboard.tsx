import { useState } from 'react';
import { Navbar, Tooltip, UnstyledButton, createStyles, Stack } from '@mantine/core';
import {
    IconHome2,
    IconGauge,
    IconDeviceDesktopAnalytics,
    IconFingerprint,
    IconCalendarStats,
    IconUser,
    IconSettings,
    IconLogout,
    IconSwitchHorizontal,
    TablerIconsProps,
} from '@tabler/icons-react';

const useStyles = createStyles((theme) => ({
    link: {
        width: 50,
        height: 50,
        borderRadius: theme.radius.md,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.colors.gray[7],

        '&:hover': {
            backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[5] : theme.colors.gray[0],
        },
    },

    active: {
        '&, &:hover': {
            backgroundColor: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).background,
            color: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).color,
        },
    },
}));


interface DashboardButtonProps {
    label: string;
    active?: boolean;
    onClick?(): void;
    children: {};
}

function DashboardButton({ label, active, onClick }: DashboardButtonProps) {
    const { classes, cx } = useStyles();
    return (
        <Tooltip label={label} position="right" transitionDuration={0}>
            <UnstyledButton onClick={onClick} className={cx(classes.link, { [classes.active]: active })}>

            </UnstyledButton>
        </Tooltip>
    );
}

export function Dashboard() {
    const [active, setActive] = useState(2);
    const { classes } = useStyles();
    return (
        <Navbar height={750} width={{ base: 80 }} p="md">
            <Navbar.Section grow mt={50}>
                <Stack justify="center" spacing={0}>
                    <DashboardButton label='Home' active={active === 1} onClick={() => setActive(1)}>
                        <IconHome2 />
                    </DashboardButton>
                </Stack>
            </Navbar.Section>
            <Navbar.Section>
                <Stack justify="center" spacing={0}>
                    <DashboardButton label="Change account" >
                        <IconSwitchHorizontal className={classes.link} />
                    </DashboardButton>
                    <DashboardButton label="Logout" >
                        <IconLogout />
                    </DashboardButton>
                </Stack>
            </Navbar.Section>
        </Navbar>
    );
}