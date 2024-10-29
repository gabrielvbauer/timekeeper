interface AppointmentProps {
  time: string
  type: 'in' | 'out'
}

export function Appointment({ time, type }: AppointmentProps) {
  return (
    <div className="flex justify-between py-3 border-b border-zinc-500">
      <p>{time}</p>
      <p>{type}</p>
    </div>
  )
}