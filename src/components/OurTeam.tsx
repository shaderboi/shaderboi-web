const profiles = [
  {
    name: "Muhammad Gusti Nurfathin",
    subtitle: "Co-Founder",
    image: "https://avatars.githubusercontent.com/u/14961870?v=4",
    description: "Computer graphics and a C++ guru",
  },
  {
    name: "Andra Antariksa Prihadi",
    subtitle: "Co-Founder",
    image: "/img/avatar/andra.png",
    description: "Mobile and web specialist",
  },
  {
    name: "Michael",
    subtitle: "Co-Founder",
    image: "/img/avatar/michael.png",
    description: "Marketing and business professional",
  },
  {
    name: "Muhammad Nabil Asykar",
    subtitle: "Co-Founder",
    image: "/img/avatar/nabil.png",
    description: "Multimedia engineer",
  },
];

export const OurTeam = () => {
  return (
    <>
      <h2 className="text-3xl mb-2 font-semibold leading-normal tracking-tight">Our Team</h2>
      <h3 className="text-xl mb-12">
        Shaderboi is steered by four software engineer professional, marketing,
        and business experts.
      </h3>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-10 max-w-5xl mx-15 sm:mx-10">
        {profiles.map((profile, idx) => {
          return (
            <div key={idx} className="w-full">
              <img src={profile.image} className="object-cover w-full mb-3" />
              <h2 className="font-semibold text-sm">{profile.name}</h2>
              <p>{profile.description}</p>
            </div>
          );
        })}
      </div>
    </>
  );
};
