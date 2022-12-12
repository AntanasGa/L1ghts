import React from "react";
import { Outlet, Link, useLocation } from "react-router-dom";

export default function AuthedSettings () {
  const location = useLocation();
  // navigation.location?.pathname
  const linkStyle = ["py-2", "px-1", "text-end"];
  const activeLinkStyle = [...linkStyle, "font-black", "border-l-4", "border-black", "dark:border-white"];
  const linkList = [
    { to: "/settings", item: "Account"},
    { to: "/settings/devices", item: "Devices"},
  ];
  return (
    <div className="flex items-center place-content-center h-screen">
      <div className="flex items-center card self-center p-0 devide-X divide-gray-500 divide-x-2 container mx-auto mb-56 mt-4 max-h-fit">
        <section className="basis-1/4 xl:basis-2/12 p-4">
          <ul>
            {
              linkList.map(e => (
                <li key={ e.to } className={ (location.pathname === e.to ? activeLinkStyle : linkStyle).join(" ") }>
                  <Link to={ e.to }>{ e.item }</Link>
                </li>
              ))
            }
          </ul>
        </section>
        <main className="basis-3/4 xl:basis-10/12 p-4 overflow-y-auto overflow-y-auto"><Outlet /></main>
      </div>
    </div>
  );
}
