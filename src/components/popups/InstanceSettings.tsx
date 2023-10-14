import * as core from '@mantine/core'
import { SettingsDashboard } from './SettingsDashboard'

export function InstanceSettings (): JSX.Element {
  return (
        <core.Box sx={{ display: 'flex', alignItems: 'center', minHeight: '30vh', width: '80wh' }}>
            <SettingsDashboard />
        </core.Box>
  )
}
