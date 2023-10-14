import { type ReactNode, useState, useEffect, useContext } from 'react'
import { Navbar, Tooltip, UnstyledButton, createStyles, Stack } from '@mantine/core'
import {
  IconHome2,
  IconSettings
} from '@tabler/icons-react'
import { PlayerHead } from './ui/playerHead'
import { Link } from 'react-router-dom'
import { selectedAccount } from '../context/AccountContext'

const useStyles = createStyles((theme) => ({
  link: {
    width: 50,
    height: 50,
    borderRadius: theme.radius.md,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    color: theme.colorScheme === 'dark' ? theme.colors.dark[0] : theme.colors.gray[7],
    transition: '300ms',
    '&:hover': {
      backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[5] : theme.colors.gray[0]
    }
  },

  active: {
    '&, &:hover': {
      backgroundColor: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).background,
      color: theme.fn.variant({ variant: 'light', color: theme.primaryColor }).color
    }
  }
}))

interface DashboardButtonProps {
  Icon: ReactNode
  label: string
  active?: boolean
  onClick?: () => void
}

function DashboardButton ({ Icon, label, active, onClick }: DashboardButtonProps): JSX.Element {
  const { classes, cx } = useStyles()
  return (
        <Tooltip label={label} position="right" transitionDuration={0}>
            <UnstyledButton onClick={onClick} className={cx(classes.link, { [classes.active]: active })}>
                {Icon}
            </UnstyledButton>
        </Tooltip>
  )
}

export function Dashboard (): JSX.Element {
  const [active, setActive] = useState(0)
  const { nickname } = useContext(selectedAccount)

  function setActiveLocation (): void {
    if (window.location.pathname === '/accounts') {
      setActive(0)
    }
    if (window.location.pathname === '/') {
      setActive(1)
    }
    if (window.location.pathname === '/settings') {
      setActive(2)
    }
  }

  useEffect(() => {
    setActiveLocation()
  })

  useStyles()
  return (
        <Navbar height="100vh" width={{ base: 80 }} p="md" sx={{ border: 'none' }}>
            <Navbar.Section>
                <Link to='/accounts' onClick={() => { setActive(0) }}>
                    <PlayerHead nickname={nickname} />
                </Link>
            </Navbar.Section>
            <Navbar.Section grow mt={32}>
                <Stack justify="center" spacing={8}>
                    <Link to=''>
                        <DashboardButton Icon={<IconHome2 />} label='Home' active={active === 1} onClick={() => { setActive(1) }} />
                    </Link>
                    <Link to='settings'>
                        <DashboardButton Icon={<IconSettings />} label='Settings' active={active === 2} onClick={() => { setActive(2) }} />
                    </Link>
                </Stack>
            </Navbar.Section>
        </Navbar >
  )
}
