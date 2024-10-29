import { useEffect, useState } from "react";
import "./App.css";
import { Appointment } from "./components/appointment";
import { Button } from "./components/ui/button";
import { read, write } from "./utils/json-handler";

const FILE_PATH = 'appointments.json';

type Appointment = {
  time: string
  type: 'in' | 'out'
}

type Appointments = {
  appointments: Appointment[]
}

export function App() {
  const [appointments, setAppointments] = useState<Appointment[]>([]);
  const [teste, setTeste] = useState('');

  async function makeAppointment() {
    console.log('opa')
    try {
      await write<Appointments>(FILE_PATH, {
        appointments: [{
          time: '10:00',
          type: 'in'
        }]
      });
    } catch (err) {
      console.log('a')
      setTeste(err as any)
    }
  }

  return (
    <main className="px-6 py-6 h-screen flex flex-col justify-between">
      <p>{teste}</p>
      {
        appointments?.map(appointment => {
          return (
            <Appointment time={appointment.time} type={appointment.type} />
          )
        })
      }
      <Button className="mt-auto" onClick={makeAppointment}>Shift</Button>
    </main>
  )
}
