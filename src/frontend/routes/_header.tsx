export default function Header() {
  return (
    <header class="bg-blue-300 md:flex md:items-center md:justify-between md:p-6">
      <span class="text-sm font-semibold text-gray-500 sm:text-center ">
        Pyum
      </span>
      <div class="ml-4 block flex-grow lg:flex lg:items-center lg:w-auto">
        <div class="text-sm lg:flex-grow text-gray-500">
          <a
            href="/device"
            class="block mt-4 lg:inline-block lg:mt-0 hover:text-white mr-4"
          >
            Devices
          </a>
          <a
            href="/sensor_purpose"
            class="block mt-4 lg:inline-block lg:mt-0 hover:text-white mr-4"
          >
            SensorPurpose
          </a>
          <a
            href="/sensor"
            class="block mt-4 lg:inline-block lg:mt-0 hover:text-white mr-4"
          >
            Sensor
          </a>
        </div>
      </div>
      <div>
        <a
          href="/admin_user/login"
          class="inline-block text-sm px-4 py-2 leading-none border rounded text-white border-white hover:border-transparent hover:text-teal-500 hover:bg-white mt-4 lg:mt-0"
        >
          Login
        </a>
      </div>
    </header>
  );
}
