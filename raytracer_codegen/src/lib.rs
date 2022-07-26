mod vec3;

use quote::quote;
use proc_macro2::TokenStream;
use rand::Rng;
use vec3::{Vec3, random_double, random_range};

#[derive(Clone)]
struct Content {
    bounding: Vec3,
    code: TokenStream,
}

// pub fn build_from_vec_static(input: TokenStream) -> TokenStream {
    
// }

#[proc_macro]
pub fn random_scene_macro(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut objects = vec![];
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            
            let center = Vec3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            let r1 = center.x;
            let r2 = center.y;
            let r3 = center.z;

            if ((center - Vec3::new(4.0, 0.2, 0.0)).len()) > 1.19 {
            
                if choose_mat < 0.6 {
                    let albedo = Vec3::random() * Vec3::random();

                    let x1 = albedo.x;
                    let x2 = albedo.y;
                    let x3 = albedo.z;

                    let fuzz = random_range(0., 0.5);
                    
                    objects.push(Content{
                        bounding: center - Vec3::new(0.2, 0.2, 0.2),
                        code: quote! { 
                            (
                                Arc::new(Sphere::new(
                                    Vec3{x:#r1, y: #r2, z:#r3},
                                    0.2,
                                    Lambertian::<SolidColor>::new(Vec3::new(#x1, #x2, #x3)),
                                ))
                            )
                        }, // brackets ? 
                    });
                } else if choose_mat < 0.850 {
                    // print!("***");
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let x1 = albedo.x;
                    let x2 = albedo.y;
                    let x3 = albedo.z;
                    let fuzz = random_range(0., 0.5);

                    objects.push(Content{
                        bounding: center - Vec3::new(0.2, 0.2, 0.2),
                        code: quote! {
                            (
                                Arc::new(Sphere::new(
                                    Vec3{x:#r1, y: #r2, z:#r3,} ,
                                    0.2,
                                    Metal::new(Vec3::new(#x1, #x2, #x3), #fuzz)
                                ))
                            )
                        },
                    });
                } else {
                    objects.push(Content {
                        bounding: center - Vec3::new(0.2, 0.2, 0.2),
                        code: quote! {
                            (
                                Arc::new(Sphere::new(
                                    Vec3 { x: #r1, y: #r2, z: #r3, },
                                    0.2,
                                    Dielectric::new(1.5),
                                ))
                            )
                        },
                    });
                }
            }
        }
    }

    objects.push(Content {
        bounding: Vec3::new(0.3, 0.3, 0.3) - Vec3::new(0.2, 0.2, 0.2),
        code: quote! {
                (
                    Arc::new(MoveSphere {
                            center0: Vec3::new(0.1,0.1,0.1),
                            center1:Vec3::new(0.2,0.2,0.2),
                            time0: 0.0,//todo
                            time1: 1.0,
                            radius: 0.2,
                            mat: Lambertian::<SolidColor>::new(Vec3::new(0.99, 0.0, 0.0)),
                        })
                )
            },
    });

    let allnode = bvh_build(&mut objects);

    let result = proc_macro::TokenStream::from(quote! {
         fn add_bvh_static()->Arc<dyn Hittable>{
            let a=Vec3::new(0.0,0.0,0.0);
            let b=Vec3{
                x:0.0,y:0.0,z:0.0,
            };
                #allnode
            }
    });
    return result.into();
}

fn bvh_build(contenent: &mut Vec<Content>) -> TokenStream {
    let span = contenent.len();
    let mut objects = contenent.clone();
    let axis = rand::thread_rng().gen_range(0..3);
    if span == 1 {
        let left = objects.remove(0);
        left.code
    } else {
        objects.sort_by(|a, b| {
            a.bounding[axis].partial_cmp(&b.bounding[axis]).unwrap()
        });
        let mid = span / 2;
        let (object0, object1) = objects.split_at_mut(mid);
        let left = bvh_build(&mut object0.to_vec());
        let right = bvh_build(&mut object1.to_vec());

        quote! ( Arc::new(BvhNode::new_node_macro(#left, #right, 0., 1.)) )
    }
}