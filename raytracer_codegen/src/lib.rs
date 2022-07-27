#![allow(clippy::all, unused_variables, unused_imports)]

extern crate proc_macro;

mod vec3;

// use syn;
// use std::sync::Arc;
use proc_macro2::TokenStream;
use quote::quote;
use rand::Rng;
use vec3::{random_double, random_range, Vec3};

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
                (a * 15) as f64 + 5.9 * random_double(),
                0.2,
                (b * 15) as f64 + 5.9 * random_double(),
            );
            let r1 = center.x;
            let r2 = center.y;
            let r3 = center.z;
            let radius = 6.;

            objects.push(Content {
                // 这里的bounding取AABB盒的左下角
                bounding: Vec3::new(r1 - radius, r2 - radius, r3 - radius),
                code: quote! {
                    Arc::new(
                        Sphere::new(
                            Vec3::new(#r1, #r2, #r3),
                            #radius,
                            Lambertian::<SolidColor>::new(Vec3::new(0.5, 0.5, 0.1))
                    ))
                },
            });
            // if ((center - Vec3::new(4.0, 0.2, 0.0)).len()) > 1.19 {

            //     if choose_mat < 0.6 {
            //         let albedo = Vec3::random() * Vec3::random();

            //         let x1 = albedo.x;
            //         let x2 = albedo.y;
            //         let x3 = albedo.z;

            //         let fuzz = random_range(0., 0.5);

            //         objects.push(Content{
            //             bounding: center - Vec3::new(0.2, 0.2, 0.2),
            //             code: quote! {
            //                 (
            //                     Arc::new(Sphere::new(
            //                         Vec3{x:#r1, y: #r2, z:#r3},
            //                         0.2,
            //                         Lambertian::<SolidColor>::new(Vec3::new(#x1, #x2, #x3)),
            //                     ))
            //                 )
            //             }, // brackets ?
            //         });
            //     } else if choose_mat < 0.850 {
            //         // print!("***");
            //         let albedo = Vec3::random_range(0.5, 1.0);
            //         let x1 = albedo.x;
            //         let x2 = albedo.y;
            //         let x3 = albedo.z;
            //         let fuzz = random_range(0., 0.5);

            //         objects.push(Content{
            //             bounding: center - Vec3::new(0.2, 0.2, 0.2),
            //             code: quote! {
            //                 (
            //                     Arc::new(Sphere::new(
            //                         Vec3{x:#r1, y: #r2, z:#r3,} ,
            //                         0.2,
            //                         Metal::new(Vec3::new(#x1, #x2, #x3), #fuzz)
            //                     ))
            //                 )
            //             },
            //         });
            //     } else {
            //         objects.push(Content {
            //             bounding: center - Vec3::new(0.2, 0.2, 0.2),
            //             code: quote! {
            //                 (
            //                     Arc::new(Sphere::new(
            //                         Vec3 { x: #r1, y: #r2, z: #r3, },
            //                         0.2,
            //                         Dielectric::new(1.5),
            //                     ))
            //                 )
            //             },
            //         });
            //     }
            // }
        }
    }

    objects.push(Content {
        bounding: Vec3::new(10., 30., 10.) - Vec3::new(20., 20., 20.),
        code: quote! {
            (
                Arc::new(MoveSphere {
                        center0: Vec3::new(10., 30., 10.),
                        center1:Vec3::new(20., 40., 20.),
                        time0: 0.0,//todo
                        time1: 1.0,
                        radius: 20.,
                        mat: Lambertian::<SolidColor>::new(Vec3::new(0.8, 0.2, 0.2)),
                })
            )
        },
    });

    let allnode = bvh_build(&mut objects);

    let result = proc_macro::TokenStream::from(quote! {
        fn add_bvh_static() -> BvhNode {
            // Sphere::new(Vec3::new(0., 0., 0.), 50., Lambertian::<SolidColor>::new(Vec3::new(0.8, 0.3, 0.3)))
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
        let code = left.code;
        quote! ( BvhNode::new_node_macro(#code, #code, 0., 1.) )
    } else {
        objects.sort_by(|a, b| a.bounding[axis].partial_cmp(&b.bounding[axis]).unwrap());
        let mid = span / 2;
        let (object0, object1) = objects.split_at_mut(mid);
        let left = bvh_build(&mut object0.to_vec());
        let right = bvh_build(&mut object1.to_vec());

        quote! ( BvhNode::new_node_macro(Arc::new(#left), Arc::new(#right), 0., 1.) )
    }
}
