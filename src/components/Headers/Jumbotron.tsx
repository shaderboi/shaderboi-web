import React from "react";
// @ts-ignore
import ShadertoyReact from "shadertoy-react";

const fs = `
float sdCircle( in vec2 p, in float r ) 
{
    return length(p)-r;
}

float sdBox( in vec2 p, in vec2 b )
{
    vec2 d = abs(p)-b;
    return length(max(d,0.0)) + min(max(d.x,d.y),0.0);
}

float sdEquilateralTriangle( in vec2 p )
{
    const float k = sqrt(3.0);
    p.x = abs(p.x) - 1.0;
    p.y = p.y + 1.0/k;
    if( p.x+k*p.y>0.0 ) p = vec2(p.x-k*p.y,-k*p.x-p.y)/2.0;
    p.x -= clamp( p.x, -2.0, 0.0 );
    return -length(p)*sign(p.y);
}

mat2 rot(float p)
{
    float s = sin(p);
    float c = cos(p);
    return mat2(c, s, -s, c);
}

struct Distance
{
    float d;
    vec3 col;
};

Distance mi(Distance a, Distance b)
{
    if (a.d < b.d) return a;
    return b;
}

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
	vec2 p = (2.0*fragCoord-iResolution.xy)/iResolution.y;
  
	Distance d = Distance(sdBox((p * rot(iTime * .4) + vec2(0., .6)) * rot(-iTime), vec2(.3)), vec3(0.65, 0.85, 1.));
    d = mi(d, Distance(sdEquilateralTriangle((p * rot(iTime * .4) + vec2(0., -.6)) * rot(-iTime) / .3) * .3, vec3(1., 0., 0.)));
    
  vec4 col = (d.d > 0.0) ? vec4(vec3(1.), 0.) : vec4(d.col, 1.);

	fragColor = col;
}
`;

export const Jumbotron = () => {
  return (
    <section className="header relative mx-4 items-center flex py-32 max-h-860-px">
      <div className="container mx-auto items-center flex flex-col md:flex-row">
        <div className="w-full">
          <div className="pt-32 sm:pt-0">
            <h2 className="font-semibold text-4xl text-blueGray-600 tracking-tight">
              A place where you can find a solution for your problem.
            </h2>
            <p className="mt-4 text-lg leading-relaxed text-blueGray-500">
              We can realize your idea at an afforable cost.
            </p>
          </div>
        </div>
        <div className="mt-8 w-full bg-red-300 h-96">
          <ShadertoyReact fs={fs} />
        </div>
      </div>
      {/* <img
          className="absolute top-0 b-auto right-0 pt-16 sm:w-6/12 -mt-48 sm:mt-0 w-10/12 max-h-860-px"
          src="/img/pattern_nextjs.png"
          alt="..."
        /> */}
    </section>
  );
};
