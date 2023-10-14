import {
  Box
} from '@mantine/core'
import { IconBrowser } from '@tabler/icons-react'

export function TopBar (): JSX.Element {
  return (
        <Box sx={(theme) => ({
          display: 'flex',
          height: '30px',
          paddingRight: '8px',
          gap: '8px',
          justifyContent: 'end',
          alignItems: 'center',
          backgroundColor: theme.colors.dark[7]
        })}>
            <IconBrowser color='white' stroke={1.5} />
            <IconBrowser color='white' stroke={1.5} />
            <IconBrowser color='white' stroke={1.5} />
        </Box>
  )
}
