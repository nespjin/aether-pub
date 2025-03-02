import pubLogo from "~/assets/images/pub-dev-logo.svg";

export function MenuBar() {
  return (
    <div
      className="h-13 w-full px-13 flex flex-row justify-between items-center"
      style={{ background: "#1C2834" }}
    >
      <img className="size-35" alt="Logo" src={pubLogo} />
      {/* <div className="text-white text-xl font-normal">Help</div> */}
    </div>
  );
}
